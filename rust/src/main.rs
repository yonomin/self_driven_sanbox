//导入相机模块
mod camera_controller;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

#[derive(Component)]
struct Ground;

//软件起始动作
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //摄像机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15., 5., 15.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    //平面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));
    //光照
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

//绘制地面指针
fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;
    //先获取window的指针位置
    if let Some(cursor_position) = window.cursor_position()
    //同时根据相机位置和指针位置获得一条射线,发自相机,穿过指针
    && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
    //同时 若能得到 射线与平面的交点 的 距离
    && let Some(distance) = ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    {
        //平面上的点
        let point = ray.get_point(distance);
        //绘制圆形
        gizmos.circle(
            Isometry3d::new(
                point + ground.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
            ),
            0.2,
            Color::WHITE,
        );
    }
}
