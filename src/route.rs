use eframe::egui;

use crate::{
    pages::{WifiList, WifiQR},
    state::State,
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    #[default]
    WifiList,
    WifiQR,
}

pub trait RouteExt {
    fn ui(&mut self, state: &mut State, ui: &mut egui::Ui);
}

impl Route {
    pub fn render(&self, state: &mut State, ui: &mut egui::Ui) {
        match self {
            Route::WifiList => WifiList::default().ui(state, ui),
            Route::WifiQR => WifiQR::default().ui(state, ui),
        }
    }

    pub fn navigate(state: &mut State, route: Route) {
        state.previous_route = Some(state.selected_route);
        state.selected_route = route;
    }

    pub fn back(state: &mut State) {
        if let Some(route) = state.previous_route {
            state.next_route = Some(state.selected_route);
            state.selected_route = route;
            state.previous_route = None;
        }
    }

    pub fn forward(state: &mut State) {
        if let Some(route) = state.next_route {
            state.next_route = None;
            state.previous_route = Some(state.selected_route);
            state.selected_route = route;
        }
    }
}
