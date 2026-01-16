use eframe::egui;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

use crate::deps;
use crate::install;
use crate::platform::{self, PackageManager};
use crate::uninstall;

use super::popups;
use super::screens;

#[derive(PartialEq)]
enum State {
    Ready,
    Installing,
    Uninstalling,
    Done,
    Removed,
    Error,
    Unsupported,
}

pub struct App {
    pm: PackageManager,
    state: State,
    logs: Vec<String>,
    log_rx: Option<Receiver<String>>,
    result_rx: Option<Receiver<Result<(), String>>>,
    show_error: bool,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let pm = platform::detect_package_manager();

        let state = if pm == PackageManager::Unknown {
            State::Unsupported
        } else if deps::all_installed() {
            State::Done
        } else {
            State::Ready
        };

        Self {
            pm,
            state,
            logs: Vec::new(),
            log_rx: None,
            result_rx: None,
            show_error: false,
        }
    }

    fn start_install(&mut self) {
        let (log_tx, log_rx) = channel();
        let (result_tx, result_rx) = channel();
        let pm = self.pm;

        self.state = State::Installing;
        self.logs.clear();
        self.log_rx = Some(log_rx);
        self.result_rx = Some(result_rx);

        thread::spawn(move || {
            let _ = result_tx.send(install::run(pm, log_tx));
        });
    }

    fn start_uninstall(&mut self) {
        let (log_tx, log_rx) = channel();
        let (result_tx, result_rx) = channel();
        let pm = self.pm;

        self.state = State::Uninstalling;
        self.logs.clear();
        self.log_rx = Some(log_rx);
        self.result_rx = Some(result_rx);

        thread::spawn(move || {
            let _ = result_tx.send(uninstall::run(pm, log_tx));
        });
    }

    fn process_logs(&mut self) {
        if let Some(rx) = &self.log_rx {
            while let Ok(msg) = rx.try_recv() {
                println!("[installer] {}", msg);
                self.logs.push(msg);
            }
        }
    }

    fn process_result(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.result_rx {
            if let Ok(result) = rx.try_recv() {
                let was_uninstalling = self.state == State::Uninstalling;
                self.log_rx = None;
                self.result_rx = None;

                match result {
                    Ok(()) => {
                        self.state = if was_uninstalling { State::Removed } else { State::Done };
                    }
                    Err(e) => {
                        println!("[installer] error: {}", e);
                        self.logs.push(format!("error: {}", e));
                        self.state = State::Error;
                        self.show_error = true;
                        ctx.copy_text(self.logs.join("\n"));
                    }
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_logs();
        self.process_result(ctx);

        if self.state == State::Installing || self.state == State::Uninstalling {
            ctx.request_repaint();
        }

        if self.show_error {
            popups::error(ctx, &mut self.show_error);
        }

        if self.state == State::Unsupported {
            if popups::unsupported(ctx) {
                std::process::exit(0);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                screens::header(ui);

                match self.state {
                    State::Ready => {
                        if screens::ready(ui) {
                            self.start_install();
                        }
                    }
                    State::Installing => screens::installing(ui),
                    State::Uninstalling => screens::uninstalling(ui),
                    State::Done => {
                        let (close, uninstall) = screens::done(ui);
                        if close {
                            std::process::exit(0);
                        }
                        if uninstall {
                            self.start_uninstall();
                        }
                    }
                    State::Removed => {
                        let (close, reinstall) = screens::removed(ui);
                        if close {
                            std::process::exit(0);
                        }
                        if reinstall {
                            self.start_install();
                        }
                    }
                    State::Error => {
                        if screens::error(ui) {
                            self.show_error = false;
                            self.state = State::Ready;
                        }
                    }
                    State::Unsupported => {}
                }
            });
        });
    }
}
