use qmetaobject::*;
use std::cell::RefCell;

#[derive(Clone)]
pub struct SpinOutcome {
    pub sector_idx: usize,
    pub display_text: String,
}

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

    engine.load_data(include_str!("../main.qml").into());
    engine.exec();
}