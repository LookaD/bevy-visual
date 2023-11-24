use bevy::prelude::*;
use bevy_egui::{egui::{self}, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (ui_system, ball_movement_system))
        .run();
}

#[derive(Component)]
struct Ball {
    speed: Vec3,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // Aggiungi qui la palla e il quadrato
}

fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in query.iter_mut() {
        transform.translation += ball.speed * time.delta_seconds();

        // Aggiungi qui la logica per far rimbalzare la palla sulle pareti
    }
}

fn ui_system(mut egui_context: EguiContexts, mut query: Query<&mut Ball>) {
    egui::Window::new("Impostazioni Palla").show(egui_context.ctx_mut(), |ui| {
        for mut ball in query.iter_mut() {
            ui.add(egui::Slider::new(&mut ball.speed.x, 0.0..=1000.0).text("Velocità X"));
            ui.add(egui::Slider::new(&mut ball.speed.y, 0.0..=1000.0).text("Velocità Y"));
        }
    });
}