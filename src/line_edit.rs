use qt_core::{ContextMenuPolicy, QString, WidgetAttribute};
use qt_widgets::{
    cpp_core::MutPtr,
    cpp_core::{CppBox, StaticUpcast},
    QAction, QLayout, QLineEdit, QMenu,
};
use rustqt_utils::qs;

/// Create a line_edit
pub fn create<I>(hlayout_ptr: MutPtr<I>) -> (MutPtr<QLineEdit>, CppBox<QMenu>, MutPtr<QAction>)
where
    I: StaticUpcast<QLayout>,
{
    unsafe {
        let mut package_line_edit = QLineEdit::new();
        package_line_edit.set_attribute_2a(WidgetAttribute::WAMacShowFocusRect, false);
        package_line_edit.set_object_name(&qs("PackageLineEdit"));
        package_line_edit.set_clear_button_enabled(true);
        package_line_edit.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
        let mut line_edit_popup_menu = QMenu::new();
        //let _line_edit_popup_menu_ptr = line_edit_popup_menu.as_mut_ptr();
        let choose_line_edit_clear_action =
            line_edit_popup_menu.add_action_q_string(&QString::from_std_str("Clear"));
        let line_edit_ptr = package_line_edit.as_mut_ptr();
        I::static_upcast_mut(hlayout_ptr).add_widget(package_line_edit.into_ptr());

        (
            line_edit_ptr,
            line_edit_popup_menu,
            choose_line_edit_clear_action,
        )
    }
}
