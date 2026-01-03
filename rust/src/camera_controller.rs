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

///每点弧度 用作灵敏度
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;
///像素转行数的转换系数
pub const PIXELS_PER_LINE: f32 = 16.0;

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
    //快速移动速度系数
    pub run_speed_factor: f32,
    ///?滚轮缩放系数,可能要改名
    pub scroll_factor: f32,
    //?数据源是什么
    pub friction: f32,
    ///上下点头
    pub pitch: f32,
    ///左右摇头
    pub yaw: f32,
    ///向量速度
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
            run_speed_factor: 3.0,
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
    mut mouse_cursor_grab: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    //两帧之间经过的时间
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
    //?可能是没用的变量
    let mut scroll = 0.0;
    //借滚轮单位确定一帧的滚动量
    let amount = match accumulated_mouse_scroll.unit {
        MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / PIXELS_PER_LINE,
    };
    //?累加单次滚动量
    scroll += amount;
    //通过滚轮调节相机移动速度
    controller.walk_speed += scroll * controller.scroll_factor * controller.walk_speed;
    controller.run_speed = controller.walk_speed * controller.run_speed_factor;

    //处理键盘输入
    ///?这可能是相机每帧的移动数据缓冲
    let mut axis_input = Vec3::ZERO;
    if key_input.pressed(controller.key_forward) {
        //?此处z可能要-=1.0
        axis_input.z += 1.0;
    }
    if key_input.pressed(controller.key_backward) {
        //?此处z可能要+=1.0
        axis_input.z -= 1.0;
    }
    if key_input.pressed(controller.key_right) {
        axis_input.x += 1.0;
    }
    if key_input.pressed(controller.key_left) {
        axis_input.x -= 1.0;
    }
    if key_input.pressed(controller.key_up) {
        axis_input.y += 1.0;
    }
    if key_input.pressed(controller.key_down) {
        axis_input.y -= 1.0;
    }

    //?每帧重置, 指针捕获状态改变
    //?搞清楚just_pressed函数
    //我现在的理解是,它能在上一帧松开,本帧按下时被触发
    //但时,储存上一帧状态的容器在哪?
    //帧的切换也不由我控制,与储存上帧状态的容器一起,被bevy给托管了吗?
    let mut cursor_grab_change = false;
    if key_input.just_pressed(controller.keyboard_key_to_toggle_cursor_grab) {
        *toggle_cursor_grab = !*toggle_cursor_grab;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_pressed(controller.mouse_key_to_grab_cursor) {
        *mouse_cursor_grab = true;
        cursor_grab_change = true;
    }
    if mouse_button_input.just_released(controller.mouse_key_to_grab_cursor) {
        *mouse_cursor_grab = false;
        cursor_grab_change = true;
    }
    let cursor_grab = *mouse_cursor_grab || *toggle_cursor_grab;

    //?看看这个run函数是不是每帧只会被触发一次,为什么
    //如果本帧相机移动缓冲不为零
    if axis_input != Vec3::ZERO {
        //最大速度: 如果按下奔跑键则为奔跑速度,没有按则为走路速度
        let max_speed = if key_input.pressed(controller.key_run) {
            controller.run_speed
        } else {
            controller.walk_speed
        };
        controller.velocity = axis_input.normalize() * max_speed;
        //如果移动缓冲为零
    } else {
        let friction = controller.friction.clamp(0.0, 1.0);
        //应用了摩擦力的向量速度
        controller.velocity *= 1.0 - friction;
        //设置速度死区0.000001,速度小于它则归零
        if controller.velocity.length_squared() < 1e-6 {
            controller.velocity = Vec3::ZERO;
        }
    }
    //应用速度向量
    //如果速度向量不为零
    if controller.velocity != Vec3::ZERO {
        //本地前方
        let forward = *transform.forward();
        //本地右方
        let right = *transform.right();
        //相机位置变换 先在右方向上加上距离
        transform.translation += controller.velocity.x * dt * right
            //在纵方向加上距离
            + controller.velocity.y * dt * Vec3::Y
            //在前方向加上距离
            + controller.velocity.z * dt * forward;
    }
    //如果指针捕捉状态改变
    if cursor_grab_change {
        //如果在捕捉
        if cursor_grab {
            //对于每个窗口实体中的window和cursor_options
            for (window, mut cursor_options) in &mut windows {
                //如果当前窗口不是活动窗口
                if !window.focused {
                    //则跳过该窗口
                    continue;
                }
                //反之是活动窗口,则锁定鼠标
                cursor_options.grab_mode = CursorGrabMode::Locked;
                //隐藏鼠标
                cursor_options.visible = false;
            }
        //反之没在捕捉
        } else {
            //遍历每个窗口实体的cursor_options
            for (_, mut cursor_options) in &mut windows {
                //抓取模式设置为不抓取
                cursor_options.grab_mode = CursorGrabMode::None;
                //显示鼠标指针
                cursor_options.visible = true;
            }
        }
    }
    //如果本帧鼠标位移不为零且于捕捉状态
    if accumulated_mouse_motion.delta != Vec2::ZERO && cursor_grab {
        controller.pitch = (controller.pitch
            - accumulated_mouse_motion.delta.y * RADIANS_PER_DOT * controller.sensitivity)
            .clamp(-PI / 2.0, PI / 2.0);
        controller.yaw -=
            accumulated_mouse_motion.delta.x * RADIANS_PER_DOT * controller.sensitivity;
        // transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, controller.yaw, controller.pitch, 0.0);
    }
}
