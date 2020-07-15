use std::mem;
use crate::device::CURRENT_DEVICE;
use crate::framebuffer::{Framebuffer, UpdateMode};
use crate::geom::{Rectangle, CornerSpec};
use super::{View, Event, Hub, Bus, EntryKind};
use super::icon::ICONS_PIXMAPS;
use crate::input::{DeviceEvent, FingerStatus};
use crate::gesture::GestureEvent;
use crate::font::{Fonts, font_from_style, NORMAL_STYLE};
use crate::color::{TEXT_NORMAL, TEXT_INVERTED_HARD};
use crate::app::Context;

pub struct MenuEntry {
    rect: Rectangle,
    children: Vec<Box<dyn View>>,
    kind: EntryKind,
    corner_spec: Option<CornerSpec>,
    anchor: Rectangle,
    active: bool,
}

impl MenuEntry {
    pub fn new(rect: Rectangle, kind: EntryKind, anchor: Rectangle, corner_spec: Option<CornerSpec>) -> MenuEntry {
        MenuEntry {
            rect,
            children: vec![],
            kind,
            corner_spec,
            anchor,
            active: false,
        }
    }

    pub fn update(&mut self, value: bool, hub: &Hub) {
        if let Some(v) = self.kind.get() {
            if v != value {
                self.kind.set(value);
                hub.send(Event::Render(self.rect, UpdateMode::Gui)).ok();
            }
        }
    }
}

impl View for MenuEntry {
    fn handle_event(&mut self, evt: &Event, hub: &Hub, bus: &mut Bus, _context: &mut Context) -> bool {
        match *evt {
            Event::Device(DeviceEvent::Finger { status, position, .. }) => {
                match status {
                    FingerStatus::Down if self.rect.includes(position) => {
                        self.active = true;
                        hub.send(Event::Render(self.rect, UpdateMode::Fast)).ok();
                        true
                    },
                    FingerStatus::Up if self.active => {
                        self.active = false;
                        hub.send(Event::Render(self.rect, UpdateMode::Gui)).ok();
                        true
                    },
                    _ => false,
                }
            },
            Event::Gesture(GestureEvent::Tap(center)) |
            Event::Gesture(GestureEvent::HoldFingerShort(center, ..)) if self.rect.includes(center) => {
                match self.kind {
                    EntryKind::CheckBox(_, _, ref mut value) => {
                        *value = !*value;
                        hub.send(Event::Render(self.rect, UpdateMode::Gui)).ok();
                    },
                    EntryKind::RadioButton(_, _, ref mut value) if !*value => {
                        *value = true;
                        hub.send(Event::Render(self.rect, UpdateMode::Gui)).ok();
                    },
                    _ => (),
                };
                match self.kind {
                    EntryKind::Command(_, ref id) |
                    EntryKind::CheckBox(_, ref id, _) |
                    EntryKind::RadioButton(_, ref id, _) => {
                        bus.push_back(Event::Select(id.clone()));
                        if let Event::Gesture(GestureEvent::Tap { .. }) = *evt {
                            bus.push_back(Event::Validate);
                        }
                    },
                    EntryKind::SubMenu(_, ref entries) => {
                        bus.push_back(Event::SubMenu(self.anchor, entries.clone()));
                    },
                    EntryKind::Message(_) => {
                        bus.push_back(Event::Validate);
                    },
                    _ => (),
                };
                true
            },
            Event::PropagateSelect(ref other_id) => {
                match self.kind {
                    EntryKind::RadioButton(_, ref id, ref mut value) if *value => {
                        if mem::discriminant(id) == mem::discriminant(other_id) && id != other_id {
                            *value = false;
                            hub.send(Event::Render(self.rect, UpdateMode::Gui)).ok();
                            true
                        } else {
                            false
                        }
                    },
                    _ => false,
                }
            },
            _ => false,
        }
    }

    fn render(&self, fb: &mut dyn Framebuffer, _rect: Rectangle, fonts: &mut Fonts) {
        let dpi = CURRENT_DEVICE.dpi;
        let font = font_from_style(fonts, &NORMAL_STYLE, dpi);
        let x_height = font.x_heights.0 as i32;
        let padding = 4 * font.em() as i32;

        let scheme = if self.active {
            TEXT_INVERTED_HARD
        } else {
            TEXT_NORMAL
        };

        if let Some(ref cs) = self.corner_spec {
            fb.draw_rounded_rectangle(&self.rect, cs, scheme[0]);
        } else {
            fb.draw_rectangle(&self.rect, scheme[0]);
        }

        let max_width = self.rect.width() as i32 - padding;
        let plan = font.plan(self.kind.text(), Some(max_width), None);
        let dy = (self.rect.height() as i32 - x_height) / 2;
        let pt = pt!(self.rect.min.x + padding / 2,
                     self.rect.max.y - dy);

        font.render(fb, scheme[1], &plan, pt); 

        let (icon_name, x_offset) = match self.kind {
            EntryKind::CheckBox(_, _, value) if value => ("check_mark", 0),
            EntryKind::RadioButton(_, _, value) if value => ("bullet", 0),
            EntryKind::SubMenu(..) => ("angle-right-small",
                                       self.rect.width() as i32 - padding / 2),
            _ => ("", 0),
        };

        if let Some(pixmap) = ICONS_PIXMAPS.get(icon_name) {
            let dx = x_offset + (padding / 2 - pixmap.width as i32) / 2;
            let dy = (self.rect.height() as i32 - pixmap.height as i32) / 2;
            let pt = self.rect.min + pt!(dx, dy);

            fb.draw_blended_pixmap(pixmap, pt, scheme[1]);
        }
    }

    fn rect(&self) -> &Rectangle {
        &self.rect
    }

    fn rect_mut(&mut self) -> &mut Rectangle {
        &mut self.rect
    }

    fn children(&self) -> &Vec<Box<dyn View>> {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn View>> {
        &mut self.children
    }
}
