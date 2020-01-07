use pbgui_toolbar::toolbar;
use pbgui_toolbar::*;
use qt_core::QResource;
use qt_widgets::{QApplication, QMainWindow, QWidget};
use rustqt_utils::{create_vlayout, load_stylesheet, qs};

fn main() {
    QApplication::init(|_app| unsafe {
        let _result = QResource::register_resource_q_string(&qs("/Users/jgerber/bin/pbgui.rcc"));
        let mut main_window = QMainWindow::new_0a();
        let mut main_widget = QWidget::new_0a();
        let main_widget_ptr = main_widget.as_mut_ptr();

        // main_layout

        let mut main_layout = create_vlayout();
        let main_layout_ptr = main_layout.as_mut_ptr();
        main_widget.set_layout(main_layout.into_ptr());

        // set main_widget as the central widget in main_window
        main_window.set_central_widget(main_widget.into_ptr());
        let tb = toolbar::create(&mut main_window.as_mut_ptr());
        load_stylesheet(
            "/Users/jgerber/src/rust/pbgui-toolbar/resources/toolbar.qss",
            main_window.as_mut_ptr(),
        );
        main_window.show();
        QApplication::exec()
    });
}
