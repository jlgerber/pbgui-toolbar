use crate::{combo_boxes, line_edit, query_button};
use qt_core::{AlignmentFlag, QFlags, QString};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QAction, QComboBox, QFrame, QLineEdit, QMainWindow, QMenu, QPushButton, QToolBar,
};

use rustqt_utils::{create_hlayout, qs, set_stylesheet_from_str};

/// The main toolbar structure
pub struct MainToolbar {
    pub toolbar: MutPtr<QToolBar>,
    pub query_btn: MutPtr<QPushButton>,
    pub level: MutPtr<QComboBox>,
    pub role: MutPtr<QComboBox>,
    pub platform: MutPtr<QComboBox>,
    pub site: MutPtr<QComboBox>,
    pub dir: MutPtr<QComboBox>,
    pub line_edit: MutPtr<QLineEdit>,
    pub menu: CppBox<QMenu>,
    pub clear_line_edit_action: MutPtr<QAction>,
}

/// load style at compile time
const STYLE_STR: &'static str = include_str!("../resources/toolbar.qss");

/// Create the MainToolbar structure
pub fn create(main_window: &mut MutPtr<QMainWindow>) -> MainToolbar {
    unsafe {
        let mut top_toolbar = main_window.add_tool_bar_q_string(&qs("TopToolBar"));
        top_toolbar.set_floatable(false);
        top_toolbar.set_movable(false);
        let mut hlayout = create_hlayout();
        let mut hlayout_ptr = hlayout.as_mut_ptr();
        let query_btn = query_button::create("", hlayout.as_mut_ptr());

        let (level, role, platform, site, dir) = combo_boxes::create(&mut hlayout_ptr);

        let (line_edit, menu, clear_line_edit_action) = line_edit::create(hlayout.as_mut_ptr());
        let _align: QFlags<AlignmentFlag> = AlignmentFlag::AlignCenter.into();
        // hlayout.set_alignment_q_layout_q_flags_alignment_flag(hlayout_ptr, align);
        // hlayout.set_alignment_q_widget_q_flags_alignment_flag(level, align);

        let mut toolbar_frame = QFrame::new_0a();
        toolbar_frame.set_object_name(&qs("ToobarFrame"));
        toolbar_frame.set_layout(hlayout.into_ptr());
        top_toolbar.add_widget(toolbar_frame.into_ptr());

        MainToolbar {
            toolbar: top_toolbar,
            query_btn,
            level,
            role,
            platform,
            site,
            dir,
            line_edit,
            menu,
            clear_line_edit_action,
        }
    }
}

impl MainToolbar {
    /// Get a mutable reference to the mutable pointer to the level
    /// combobox.
    pub fn level(&mut self) -> MutPtr<QComboBox> {
        self.level
    }

    /// set the levels to choose from in the combobox's dropdown list
    pub fn set_level_items<I: AsRef<str>>(&mut self, inputs: Vec<I>) {
        unsafe {
            inputs
                .iter()
                .filter(|s| s.as_ref() != "facility")
                .for_each(|s| {
                    self.level
                        .add_item_q_string(&QString::from_std_str(s.as_ref()))
                });
        }
    }

    /// add a level to choose from in the combobox's dropdown list
    pub fn add_level_item<I: AsRef<str>>(&mut self, input: I) {
        unsafe {
            self.level
                .add_item_q_string(&QString::from_std_str(input.as_ref()))
        }
    }

    /// Get a mutable reference to the mutable pointer to the role
    /// combobox.
    pub fn role(&mut self) -> MutPtr<QComboBox> {
        self.role
    }

    /// Set role items in the combobox dropdown list
    pub fn set_role_items<I: AsRef<str>>(&mut self, inputs: Vec<I>) {
        unsafe {
            inputs.iter().filter(|s| s.as_ref() != "any").for_each(|s| {
                self.role
                    .add_item_q_string(&QString::from_std_str(s.as_ref()))
            });
        }
    }

    /// add a role to choose from in the combobox's dropdown list
    pub fn add_role_item<I: AsRef<str>>(&mut self, input: I) {
        unsafe {
            self.role
                .add_item_q_string(&QString::from_std_str(input.as_ref()))
        }
    }

    /// Get a mutable reference to the mutable pointer to the platform
    /// combobox.
    pub fn platform(&mut self) -> MutPtr<QComboBox> {
        self.platform
    }

    /// Set the platforms to choose from in the combobox's dropdown list
    pub fn set_platform_items<I: AsRef<str>>(&mut self, inputs: Vec<I>) {
        unsafe {
            inputs.iter().filter(|s| s.as_ref() != "any").for_each(|s| {
                self.platform
                    .add_item_q_string(&QString::from_std_str(s.as_ref()))
            });
        }
    }

    /// Get a mutable reference to the mutable pointer to the site
    /// combobox.
    pub fn site(&mut self) -> MutPtr<QComboBox> {
        self.site
    }

    /// Set the sites to choose from in the combobox's dropdown list.
    pub fn set_site_items<I: AsRef<str>>(&mut self, inputs: Vec<I>) {
        unsafe {
            inputs.iter().filter(|s| s.as_ref() != "any").for_each(|s| {
                self.site
                    .add_item_q_string(&QString::from_std_str(s.as_ref()))
            });
        }
    }

    /// add a site to choose from in the combobox's dropdown list
    pub fn add_site_item<I: AsRef<str>>(&mut self, input: I) {
        unsafe {
            self.site
                .add_item_q_string(&QString::from_std_str(input.as_ref()))
        }
    }

    /// Get a mutable reference to the mutable pointer to the dir
    /// combobox.
    pub fn dir(&mut self) -> MutPtr<QComboBox> {
        self.dir
    }

    /// Retrieve a query button
    pub fn query_btn(&mut self) -> MutPtr<QPushButton> {
        self.query_btn
    }

    /// Set the stylesheet to the internal stylesheet
    pub fn set_default_stylesheet(&mut self) {
        set_stylesheet_from_str(STYLE_STR, self.toolbar);
    }
}
