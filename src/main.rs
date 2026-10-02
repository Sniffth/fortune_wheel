use qmetaobject::*;
use std::cell::RefCell;

#[derive(Clone)]
pub struct SpinOutcome {
    pub sector_idx: usize,
    pub display_text: String,
}

const QML_SRC: &str = r###"import QtQuick
import QtQuick.Controls


ApplicationWindow {
    visible: true
    width: 600
    height: 700
    title: "Поле чудес"
    id: root

    
    Shortcut {
        sequence: "F11"
        onActivated: {
            if (root.visibility === Window.FullScreen) {
                root.visibility = Window.Windowed
            } else {
                root.visibility = Window.FullScreen
            }
        }
    }

    Shortcut {
        sequence: "Esc"
        onActivated: {
            root.visibility = Window.Windowed
        }
    }

    property var sectorColors: ["#FF4D4D", "#4D79FF", "#FF00FF", "#FF8000", "#00FF00"]
    property var sectorNames: ["Результативный ход", "Музыкальная пауза", "Сектор Плюс", "Сектор Шанс", "Сектор Приз"]
    property string displayText: "Крутите колесо!"

    NumberAnimation {
        id: spinAnimation
        target: wheelContainer
        property: "rotation"
        duration: 3500
        easing.type: Easing.OutCubic
        onStopped: {
            displayText = rustWheel.sector_text
        }
    }

    Column {
        anchors.centerIn: parent
        spacing: 20

        Canvas {
            width: 24
            height: 30
            anchors.horizontalCenter: parent.horizontalCenter
            z: 2

            onPaint: {
                var ctx = getContext("2d");
                ctx.reset();
                ctx.fillStyle = "red";
                ctx.beginPath();
                ctx.moveTo(0, 0);
                ctx.lineTo(width, 0);
                ctx.lineTo(width / 2, height);
                ctx.closePath();
                ctx.fill();
            }
        }

        Rectangle {
            width: 400
            height: 400
            radius: 200
            border.color: "black"
            border.width: 4
            clip: false

            Item {
                id: wheelContainer
                anchors.fill: parent
                rotation: 0

                Repeater {
                    model: 5
                    delegate: Canvas {
                        width: 400
                        height: 400
                        rotation: index * 72

                        onPaint: {
                            var ctx = getContext("2d");
                            ctx.reset();
                            ctx.beginPath();
                            ctx.moveTo(200, 200);
                            ctx.arc(200, 200, 200, -Math.PI / 2, -Math.PI / 2 + (72 * Math.PI / 180), false);
                            ctx.lineTo(200, 200);
                            ctx.fillStyle = sectorColors[index];
                            ctx.fill();
                            ctx.lineWidth = 2;
                            ctx.strokeStyle = "black";
                            ctx.stroke();
                        }
                    }
                }

                Repeater {
                    model: 5
                    delegate: Item {
                        width: 400
                        height: 400
                        rotation: index * 72 + 36

                        Text {
                            text: sectorNames[index]
                            y: 30
                            anchors.horizontalCenter: parent.horizontalCenter
                            font.bold: true
                            font.pixelSize: 16
                            color: "black"
                        }
                    }
                }
            }
        }

        Button {
            text: "Крутить"
            anchors.horizontalCenter: parent.horizontalCenter
            enabled: !spinAnimation.running

            onClicked: {
                displayText = "Барабан крутится..."
                rustWheel.spin_to_next_scripted()
                spinAnimation.from = wheelContainer.rotation
                spinAnimation.to = rustWheel.rotation_angle
                spinAnimation.start()
            }
        }

        Rectangle {
            width: parent.width * 0.8
            height: 70
            color: "#2b2b3b"
            radius: 10
            border.color: "#ffcc00"
            border.width: 2
            anchors.horizontalCenter: parent.horizontalCenter

            Text {
                anchors.centerIn: parent
                width: parent.width - 20
                text: displayText
                color: "#ffffff"
                font.pixelSize: 18
                font.bold: true
                wrapMode: Text.WordWrap
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }
        }
    }
}"###;


#[derive(QObject, Default)]
pub struct FortuneWheel {
    base: qt_base_class!(trait QQuickItem),

    rotation_angle: qt_property!(f64; NOTIFY rotation_angle_changed),
    rotation_angle_changed: qt_signal!(),

    sector_text: qt_property!(String; NOTIFY sector_text_changed),
    sector_text_changed: qt_signal!(),

    spin_to_next_scripted: qt_method!(fn(&mut self) -> String),

    scripted_outcomes: RefCell<Vec<SpinOutcome>>,
    current_step: RefCell<usize>,
}
impl QQuickItem for FortuneWheel {}

impl FortuneWheel {
    pub fn new() -> Self {
        let outcomes = vec![
            SpinOutcome { sector_idx: 0, display_text: "Отгадали букву, возврат на колесо".into() },
            SpinOutcome { sector_idx: 1, display_text: "Поздравления: танцевальный номер от 9 классов".into() },
            SpinOutcome { sector_idx: 4, display_text: "Сектор Приз".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 1, display_text: "Поздравления: вокальный номер от 9 классов".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 4, display_text: "Сектор Приз".into() },
            SpinOutcome { sector_idx: 1, display_text: "Караоке".into() },
            SpinOutcome { sector_idx: 2, display_text: "Сектор Плюс — открываем любую букву".into() },
            SpinOutcome { sector_idx: 1, display_text: "Поздравления: танцевальный номер от 10А класса".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 4, display_text: "Сектор Приз".into() },
            SpinOutcome { sector_idx: 3, display_text: "Сектор Шанс — звонок или помощь зала".into() },
            SpinOutcome { sector_idx: 1, display_text: "Поздравления: танцевальный номер от 10Б класса".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 4, display_text: "Сектор Приз (Супер-приз!)".into() },
            SpinOutcome { sector_idx: 2, display_text: "Сектор Плюс".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 1, display_text: "Поздравления: номер от студии «МЮЗИК СИТИ»".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 4, display_text: "Сектор Приз".into() },
            SpinOutcome { sector_idx: 2, display_text: "Сектор Плюс".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 4, display_text: "Сектор Приз".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 3, display_text: "Сектор Шанс".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 1, display_text: "Поздравления: номер от студии ДАРИ...".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 4, display_text: "Сектор Приз".into() },
            SpinOutcome { sector_idx: 1, display_text: "Поздравления: номер от 11 класса".into() },
            SpinOutcome { sector_idx: 2, display_text: "Сектор Плюс".into() },
            SpinOutcome { sector_idx: 0, display_text: "Результативный ход".into() },
            SpinOutcome { sector_idx: 1, display_text: "Поздравления: номер от учителей".into() },
        ];

        Self {
            scripted_outcomes: RefCell::new(outcomes),
            current_step: RefCell::new(0),
            sector_text: "Нажмите «Крутить», чтобы начать".into(),
            ..Default::default()
        }
    }

    pub fn spin_to_next_scripted(&mut self) -> String {
        let outcome = {
            let outcomes = self.scripted_outcomes.borrow();
            let mut step = self.current_step.borrow_mut();

            if outcomes.is_empty() {
                return "Список результатов пуст".into();
            }

            let item = outcomes[*step % outcomes.len()].clone();
            *step += 1;
            item
        };

        self.spin_to_sector(outcome.sector_idx);

        self.sector_text = outcome.display_text.clone();
        self.sector_text_changed();

        outcome.display_text
    }

    pub fn spin_to_sector(&mut self, sector_idx: usize) {
        if sector_idx >= 5 {
            return;
        }

        let sector_angle = 360.0 / 5.0;
        let sector_target = 360.0 - (sector_idx as f64 * sector_angle + sector_angle / 2.0);
        let extra_spins = 360.0 * 5.0;

        let current_base = (self.rotation_angle / 360.0).floor() * 360.0;
        self.rotation_angle = current_base + extra_spins + sector_target;

        self.rotation_angle_changed();
    }
}

fn main() {
    qml_register_type::<FortuneWheel>(
        c"FortuneWheel",
        1,
        0,
        c"FortuneWheel",
    );

    let mut engine = QmlEngine::new();

    let wheel = QObjectBox::new(FortuneWheel::new());
    engine.set_object_property("rustWheel".into(), wheel.pinned());

    engine.load_data(QML_SRC.into());

    engine.exec();
}
