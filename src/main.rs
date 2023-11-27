use bevy::{ prelude::*, sprite::MaterialMesh2dBundle };
use bevy_egui::{ egui::{self}, EguiContexts, EguiPlugin };

#[derive(Resource)]
struct SquareBounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // Aggiungi la resource SquareBounds
        .insert_resource(SquareBounds {
            left: -300.0,   // metà di 600
            right: 300.0,   // metà di 600
            top: 300.0,     // metà di 600
            bottom: -300.0, // metà di 600
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (ui_system, ball_movement_system))
        .run();
}

#[derive(Component, Debug)]
struct Ball {
    speed: Vec3,
    radius: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window>, 
    square_bounds: Res<SquareBounds>,
) {
    // Get the window
    let window = windows.single_mut();

    // Setup camera
    commands.spawn(Camera2dBundle::default());

   
    // Dimensioni e posizione del riquadro
    let border_thickness = 5.0;
    let square_size = square_bounds.right - square_bounds.left;

    // left: -300.0,   // metà di 600
    // right: 300.0,   // metà di 600
    // top: 300.0,     // metà di 600
    // bottom: -300.0, // metà di 600

    // Creazione dei bordi del riquadro (superiore, inferiore, sinistro, destro)
    let borders = [
        // Superiore
        (Vec3::new(0., square_bounds.top, 0.0), square_size + border_thickness, border_thickness),
        // Inferiore
        (Vec3::new(0., square_bounds.bottom, 0.0), square_size + border_thickness, border_thickness),
        // Sinistro
        (Vec3::new(square_bounds.left, 0., 0.0), border_thickness, square_size),
        // Destro
        (Vec3::new(square_bounds.right, 0., 0.0), border_thickness, square_size),
    ];

    for (position, width, height) in borders.iter() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(*width, *height)).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_translation(*position),
            ..default()
        });
    }
    
    // Aggiungi la palla
    // Circle
    let ball_radius = 15.;
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(ball_radius).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    })
    .insert(Ball { speed: Vec3::new(200.0, 100.0, 0.0), radius: ball_radius });

}

fn ball_movement_system(
    // mut windows: Query<&mut Window>, 
    time: Res<Time>, 
    square_bounds: Res<SquareBounds>,
    mut query: Query<(&mut Ball, &mut Transform)>
) {
    // let window = windows.single_mut();

    for (mut ball, mut transform) in query.iter_mut() {

        // Gestione collisioni Future Position
        // da controllare se funziona correttamente
        // ci sono dei problemi a velocità elevate 
        let delta_time = time.delta_seconds();
        let future_position = transform.translation + ball.speed * delta_time;

        // Gestione delle collisioni migliorata
        if future_position.x - ball.radius <= square_bounds.left || future_position.x + ball.radius >= square_bounds.right {
            ball.speed.x = -ball.speed.x;
            // Aggiusta la posizione per evitare che la palla vada fuori dai limiti
            transform.translation.x = (square_bounds.right - ball.radius).min((square_bounds.left + ball.radius).max(transform.translation.x));
        }
        if future_position.y - ball.radius <= square_bounds.bottom || future_position.y + ball.radius >= square_bounds.top {
            ball.speed.y = -ball.speed.y;
            // Aggiusta la posizione per evitare che la palla vada fuori dai limiti
            transform.translation.y = (square_bounds.top - ball.radius).min((square_bounds.bottom + ball.radius).max(transform.translation.y));
        }

        // Aggiorna la posizione
        transform.translation += ball.speed * delta_time;

        //OLD
        // transform.translation += ball.speed * time.delta_seconds();

        // // Usa i limiti definiti in SquareBounds per il controllo delle collisioni
        // if transform.translation.x - ball.radius <= square_bounds.left || transform.translation.x + ball.radius >= square_bounds.right {
        //     ball.speed.x = -ball.speed.x;
        // }
        // if transform.translation.y - ball.radius <= square_bounds.bottom || transform.translation.y + ball.radius >= square_bounds.top {
        //     ball.speed.y = -ball.speed.y;
        // }
        
        // OLD OLD
        // transform.translation += ball.speed * time.delta_seconds();
        //
        // // Controllo collisioni con le pareti orizzontali
        // if transform.translation.x >= (window_width / 2.0) - ball.radius || transform.translation.x <= (-window_width / 2.0) + ball.radius {
        //     ball.speed.x = -ball.speed.x;
        // }

        // // Controllo collisioni con le pareti verticali
        // if transform.translation.y >= (window_height / 2.0) - ball.radius || transform.translation.y <= (-window_height / 2.0) + ball.radius {
        //     ball.speed.y = -ball.speed.y;
        // }
    }

}

fn ui_system(mut egui_context: EguiContexts, mut query: Query<&mut Ball>) {
    egui::Window::new("Impostazioni Palla").show(egui_context.ctx_mut(), |ui| {
        for mut ball in query.iter_mut() {
            // Calcola il valore assoluto della velocità
            let mut speed_x_abs = ball.speed.x.abs();
            let mut speed_y_abs = ball.speed.y.abs();

            // Mostra gli slider per i valori assoluti della velocità
            ui.add(egui::Slider::new(&mut speed_x_abs, 0.0..=10000.0).text("Velocità X"));
            ui.add(egui::Slider::new(&mut speed_y_abs, 0.0..=10000.0).text("Velocità Y"));

            // Aggiorna la velocità mantenendo il segno originale
            ball.speed.x = ball.speed.x.signum() * speed_x_abs;
            ball.speed.y = ball.speed.y.signum() * speed_y_abs;
        }
    });
}