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
    // Aggiungi la palla
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(30.0, 30.0)),
            color: Color::rgb(1.0, 0.5, 0.5),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Ball { speed: Vec3::new(100.0, 100.0, 0.0) });

    // Aggiungi qui eventualmente altri elementi
}

fn ball_movement_system(
    mut windows: Query<&mut Window>, 
    time: Res<Time>, 
    mut query: Query<(&mut Ball, &mut Transform)>
) {
    let window = windows.single_mut();
    let window_width = window.width() as f32;
    let window_height = window.height() as f32;

    for (mut ball, mut transform) in query.iter_mut() {
        transform.translation += ball.speed * time.delta_seconds();

        // Controllo collisioni con le pareti orizzontali
        if transform.translation.x >= window_width / 2.0 || transform.translation.x <= -window_width / 2.0 {
            ball.speed.x = -ball.speed.x;
        }

        // Controllo collisioni con le pareti verticali
        if transform.translation.y >= window_height / 2.0 || transform.translation.y <= -window_height / 2.0 {
            ball.speed.y = -ball.speed.y;
        }
    }
}

fn ui_system(mut egui_context: EguiContexts, mut query: Query<&mut Ball>) {
    egui::Window::new("Impostazioni Palla").show(egui_context.ctx_mut(), |ui| {
        for mut ball in query.iter_mut() {
            // Calcola il valore assoluto della velocità
            let mut speed_x_abs = ball.speed.x.abs();
            let mut speed_y_abs = ball.speed.y.abs();

            // Mostra gli slider per i valori assoluti della velocità
            ui.add(egui::Slider::new(&mut speed_x_abs, 0.0..=1000.0).text("Velocità X"));
            ui.add(egui::Slider::new(&mut speed_y_abs, 0.0..=1000.0).text("Velocità Y"));

            // Aggiorna la velocità mantenendo il segno originale
            ball.speed.x = ball.speed.x.signum() * speed_x_abs;
            ball.speed.y = ball.speed.y.signum() * speed_y_abs;
        }
    });
}