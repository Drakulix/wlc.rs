extern crate wlc;

use std::cmp;
use std::env;
use std::process;
use wlc::*;

struct Compositor {
    action: Action,
}

enum Action {
    None,
    Move(WeakView, Point),
    Resize(WeakView, Point, ResizeEdge::Flags),
}

impl Action {
    fn is_none(&self) -> bool {
        match *self {
            Action::None => true,
            _ => false,
        }
    }
}

impl Compositor {
    fn new() -> Compositor {
        Compositor { action: Action::None }
    }

    fn start_interactive_move(&mut self, view: &View, origin: Point) {
        let in_progess = !self.action.is_none();
        if !in_progess {
            view.bring_to_front();
            self.action = Action::Move(view.weak_reference(), origin);
        }
    }

    fn start_interactive_resize(&mut self, view: &View, edges: ResizeEdge::Flags, origin: Point) {
        let in_progess = !self.action.is_none();
        if !in_progess {
            let action_edges = if edges.is_empty() {
                let geo = view.geometry();
                let halfw = geo.origin.x + geo.size.w as i32 / 2;
                let halfh = geo.origin.y + geo.size.h as i32 / 2;

                let vertical = if origin.x < halfw {
                    ResizeEdge::Left
                } else if origin.x > halfw {
                    ResizeEdge::Right
                } else {
                    ResizeEdge::Null
                };

                let horizontal = if origin.y < halfh {
                    ResizeEdge::Left
                } else if origin.y > halfh {
                    ResizeEdge::Right
                } else {
                    ResizeEdge::Null
                };

                vertical | horizontal
            } else {
                edges
            };

            view.bring_to_front();
            view.set_state(ViewState::Resizing, true);
            self.action = Action::Resize(view.weak_reference(), origin, action_edges);
        }
    }

    fn stop_interactive_action(&mut self) {
        if let Action::Resize(ref weakview, _, _) = self.action {
            weakview.run(|view| { view.set_state(ViewState::Resizing, false); });
        };
        self.action = Action::None;
    }

    fn top_most<'a>(&mut self, output: &'a Output, offset: usize) -> Option<&'a View> {
        let views = output.views();
        match views.len() {
            0 => None,
            len => Some(views[(len - 1 + offset) % len]),
        }
    }

    fn relayout(&mut self, output: &Output) {
        // very simple layout function
        // you probably dont want to layout certain types of windows in a wm

        let size = output.virtual_resolution();
        let len = output.views().len();

        let mut toggle = false;
        let mut y = 0;

        let n = cmp::max((1 + len) / 2, 1) as u32;
        let (width, height) = (size.w / 2, size.h / n);
        let (ewidth, eheight) = (size.w - width * 2, size.h - height * n);

        for (i, view) in output.views().into_iter().enumerate() {
            match view.positioner() {
                Some(pos) => {
                    let mut size_req = pos.size();
                    if size_req.w == 0 || size_req.h == 0 {
                        size_req = view.geometry().size;
                    }

                    let mut geometry = Geometry {
                        origin: pos.anchor_rect().origin,
                        size: size_req,
                    };

                    if let Some(parent) = view.parent() {
                        let parent_geometry = parent.geometry();
                        geometry.origin.x += parent_geometry.origin.x;
                        geometry.origin.y += parent_geometry.origin.y;
                    }

                    view.set_geometry(ResizeEdge::Null, geometry);
                }
                None => {
                    let geometry = Geometry {
                        origin: Point {
                            x: if toggle { width + ewidth } else { 0 } as i32,
                            y: y,
                        },
                        size: Size {
                            w: if !toggle && i == len - 1 {
                                size.w
                            } else if toggle {
                                width
                            } else {
                                width + ewidth
                            },
                            h: if i < 2 { height + eheight } else { height },
                        },
                    };
                    view.set_geometry(ResizeEdge::Null, geometry);
                    y += if toggle { geometry.size.h } else { 0 } as i32;
                    toggle = !toggle;
                }
            }
        }
    }
}

impl wlc::Callback for Compositor {
    fn output_resolution(&mut self, output: &Output, _from: Size, _to: Size) {
        self.relayout(output);
    }

    fn view_created(&mut self, view: &View) -> bool {
        view.set_visibility(view.output().visibility());
        view.bring_to_front();
        view.focus();
        self.relayout(view.output());
        true
    }

    fn view_destroyed(&mut self, view: &View) {
        match self.top_most(view.output(), 0) {
            Some(view) => view.focus(),
            None => View::set_no_focus(),
        };
        self.relayout(view.output());
    }

    fn view_focus(&mut self, view: &View, focus: bool) {
        view.set_state(ViewState::Activated, focus);
    }

    fn view_request_move(&mut self, view: &View, origin: Point) {
        self.start_interactive_move(view, origin)
    }

    fn view_request_resize(&mut self, view: &View, edges: ResizeEdge::Flags, origin: Point) {
        self.start_interactive_resize(view, edges, origin)
    }

    fn view_request_geometry(&mut self, _view: &View, _geometry: Geometry) {
        // stub intentionally ignore geometry requests
    }

    fn keyboard_key(&mut self, view: Option<&View>, _time: u32, modifiers: Modifiers, key: Key,
                    state: KeyState)
                    -> bool {
        use wlc::input::keyboard::Keysyms;

        let sym = input::keyboard::keysym_for_key(key, Modifiers::empty());

        if state == KeyState::Pressed {
            if let Some(view) = view {
                if modifiers.mods.contains(Modifier::Ctrl) && sym == Keysyms::KEY_Q {
                    view.close();
                    return true;
                } else if modifiers.mods.contains(Modifier::Ctrl) && sym == Keysyms::KEY_Down {
                    view.send_to_back();
                    if let Some(new_view) = self.top_most(view.output(), 0) {
                        new_view.focus();
                    }
                    return true;
                }
            }

            if modifiers.mods.contains(Modifier::Ctrl) && sym == Keysyms::KEY_Escape {
                terminate();
                return true;
            } else if modifiers.mods.contains(Modifier::Ctrl) && sym == Keysyms::KEY_Return {
                process::Command::new(if env::var("TERMINAL").is_ok() {
                        env::var("TERMINAL").unwrap()
                    } else {
                        String::from("weston-terminal")
                    })
                    .spawn()
                    .expect("failed to spawn process");
                return true;
            } else if modifiers.mods.contains(Modifier::Ctrl) && sym >= Keysyms::KEY_1 &&
                      sym <= Keysyms::KEY_9 {
                Output::with_all_outputs(|outputs| {
                    let scale = (sym - Keysyms::KEY_1) + 1;
                    for output in outputs {
                        output.set_resolution(output.resolution(), scale);
                    }
                    println!("scale: {}", scale);
                });
                return true;
            }
        }

        false
    }

    fn pointer_button(&mut self, view: Option<&View>, _time: u32, modifiers: Modifiers, button: Button,
                      state: ButtonState, origin: Point)
                      -> bool {
        if state == ButtonState::Pressed {
            match view {
                Some(view) => {
                    view.focus();
                    if modifiers.mods.contains(Modifier::Ctrl) && button == Button::Left {
                        self.start_interactive_move(view, origin);
                    } else if modifiers.mods.contains(Modifier::Ctrl) && button == Button::Right {
                        self.start_interactive_resize(view, ResizeEdge::Null, origin);
                    }
                }
                None => View::set_no_focus(),
            };
        } else {
            self.stop_interactive_action()
        }

        !self.action.is_none()
    }

    fn pointer_motion(&mut self, _view: Option<&View>, _time: u32, position: Point) -> bool {
        match self.action {
            Action::Resize(ref weakview, ref grab, ref edges) => {
                weakview.run(|view| {
                    let dx = position.x - grab.x;
                    let dy = position.y - grab.y;
                    let mut geo = view.geometry();

                    let min_size = Size { w: 80, h: 40 };
                    let mut n = geo;

                    if edges.contains(ResizeEdge::Left) {
                        n.size.w = (n.size.w as i32 - dx) as u32;
                        n.origin.x += dx;
                    } else if edges.contains(ResizeEdge::Right) {
                        n.size.w = (n.size.w as i32 + dx) as u32;
                    }

                    if edges.contains(ResizeEdge::Top) {
                        n.size.h = (n.size.h as i32 - dy) as u32;
                        n.origin.y += dy;
                    } else if edges.contains(ResizeEdge::Bottom) {
                        n.size.h = (n.size.h as i32 + dy) as u32;
                    }

                    if n.size >= min_size {
                        geo = n;
                    }

                    view.set_geometry(*edges, geo);
                });
            }
            Action::Move(ref weakview, ref grab) => {
                weakview.run(|view| {
                    let dx = position.x - grab.x;
                    let dy = position.y - grab.y;
                    let mut geo = view.geometry();

                    geo.origin.x += dx;
                    geo.origin.y += dy;
                    view.set_geometry(ResizeEdge::Null, geo);
                });
            }
            Action::None => {}
        };

        input::pointer::set_position(position);
        !self.action.is_none()
    }
}

fn main() {
    wlc::init(Compositor::new()).unwrap();
}
