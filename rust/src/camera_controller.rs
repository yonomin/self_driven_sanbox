//使用 bevy货箱
use bevy::{
    //输入货箱中鼠标模块中鼠标动作结构,鼠标滚轮动作结构,鼠标滚轮单位结构
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
    picking::window,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
//使用标准货箱中32位浮点数模块中常量模块中的一切, 以及格式化宏模块
use std::{f32::consts::*, fmt, str};

///相机控制器插件
pub struct CameraControllerPlugin;

//给相机控制器插件实现插件特性
impl Plugin for CameraControllerPlugin {
    //构建函数,参数是相机插件实例和应用的可变引用
    fn build(&self, app: &mut App) {
        //每帧执行相机控制函数,添加到系统
        app.add_systems(Update, run_camera_controller);
    }
}

///?每点弧度 用作灵敏度
pub const RADIONS_PER_DOT: f32 = 1.0 / 180.0;

///相机控制器结构组件
#[derive(Component)]
pub struct CameraController {
    ///开关
    pub enabled: bool,
    ///初始化
    pub initialized: bool,

    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,

    pub mouse_key_to_grab_cursor: MouseButton,
    pub keyboard_key_to_toggle_cursor_grab: KeyCode,

    pub walk_speed: f32,
    pub run_speed: f32,
    ///?这是什么
    pub scroll_factor: f32,

    pub friction: f32,
    ///上下点头
    pub pitch: f32,
    ///左右摇头
    pub yaw: f32,
    ///和速度speed有什么区别? 加速方向吗？
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 1.0,
            key_forward: KeyCode::KeyW,
            key_backward: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::Space,
            key_down: KeyCode::ShiftLeft,
            key_run: KeyCode::ControlLeft,
            mouse_key_to_grab_cursor: MouseButton::Left,
            keyboard_key_to_toggle_cursor_grab: KeyCode::KeyG,
            walk_speed: 5.0,
            run_speed: 15.0,
            scroll_factor: 0.1,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

impl fmt::Display for CameraController {
    //f是一个写入器，fmt通过f写入到实际的输出目标
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
            Freecam Controls:
                Mouse\t- Move camera orientation
                Scroll\t- Adjust movement speed
                {:?}\t- Hold to grab cursor
                {:?}\t- Toggle cursor grab
                {:?} & {:?}\t- Fly forward & backwards
                {:?} & {:?}\t- Fly sideways left & right
                {:?} & {:?}\t- Fly up & down
                {:?}\t- Fly faster while held
            ",
            self.mouse_key_to_grab_cursor,
            self.keyboard_key_to_toggle_cursor_grab,
            self.key_forward,
            self.key_backward,
            self.key_left,
            self.key_right,
            self.key_up,
            self.key_down,
            self.key_run,
        )
    }
}

fn run_camera_controller(
    //现实时间
    time: Res<Time<Real>>,
    mut windows: Query<(&Window, &mut CursorOptions)>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut toggle_cursor_grab: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), with<Camera>>,
) {
    let dt = time.delta_secs();
    let Ok((mut transform, mut controller)) = query.single_mut() else {
        warn!("To run camera controller requires exactly one active camera");
        return;
    };
    //如果相机控制器还没有初始化
    if !controller.initialized {
        //进行初始化
        //摇头是Y轴，点头是X轴，滚筒式Z轴
        let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
        controller.yaw = yaw;
        controller.pitch = pitch;
        controller.initialized = true;
        //输出日志，利用controller实现display trait时的格式化器f
        info!("{}", *controller);
    }
    if !controller.enabled {
        warn!("Controller is not enabled");
        return;
    }
    let mut scroll = 0.0;
    let amount = match accumulated_mouse_scroll.unit{
        
    }
}
