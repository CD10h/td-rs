pub fn dummy() {}

/*
use std::any::Any;

use ntscrs::ntsc::{
    NtscEffectFullSettings, SettingDescriptor, SettingID, SettingKind, SettingsList,
};
use td_rs_top::{NumericParameter, OperatorParams, ParamInputs, ParameterManager, StringParameter};

// pub fn convert_setting_to_param(settings: SettingDescriptor) -> ParamOptions {
//     let setting_kind = settings.kind;
//     let description = settings.description;
//     let id = settings.id;
//     let label = settings.label;
//     let name = id.name();

//     let mut param_options = ParamOptions {
//         name: String::from(name),
//         label: String::from(label),
//         page: String::from("Custom"),
//         // min: todo!(),
//         // max: todo!(),
//         // min_slider: todo!(),
//         // max_slider: todo!(),
//         // clamp: todo!(),
//         // default: todo!(),
//     };

// }

#[derive(Default, Clone, Debug)]
struct NtscTopParams {
    full_settings: NtscEffectFullSettings,
}

impl NtscTopParams {
    fn register_setting_descriptor(
        &mut self,
        setting: &SettingDescriptor,
        parameter_manager: &mut ParameterManager,
    ) {
        let label = setting.label;
        let description = setting.description.unwrap_or_default();
        let kind = setting.kind;
        let id = setting.id;

        match kind {
            SettingKind::Enumeration {
                options,
                default_value,
            } => {
                let mut opt_copy = options.to_vec();
                opt_copy.sort_by(|a, b| a.index.cmp(&b.index));
                let names = opt_copy.iter().map(|mi| String::from(mi.label));
                let labels = opt_copy.cloned();

                let default_menu_item = opt_copy.iter().find(|mi| mi.index == default_value);

                let default_name = if let Some(menu) = default_menu_item {
                    menu.label
                } else {
                    opt_copy.first().unwrap().label
                };

                let param = StringParameter {
                    name: String::from(id.name()),
                    label: String::from(label),
                    default_value: String::from(default_name),
                    ..Default::default()
                };

                parameter_manager.append_menu(param, names, labels)
            }
            SettingKind::Percentage {
                logarithmic,
                default_value,
            } => {
                parameter_manager.append_float(NumericParameter {
                    name: String::from(id.name()),
                    label: String::from(label),
                    default_values: [default_value as f64, 0.0, 0.0, 0.0],
                    min_sliders: [1.0, 0.0, 0.0, 0.0],
                    min_sliders: [0.0, 0.0, 0.0, 0.0],
                    ..Default::default()
                });
            }
            SettingKind::IntRange {
                range,
                default_value,
            } => {
                parameter_manager.append_int(NumericParameter {
                    name: String::from(id.name()),
                    label: String::from(label),
                    default_values: [default_value as f64, 0.0, 0.0, 0.0],
                    min_sliders: [range.start() as f64, 0.0, 0.0, 0.0],
                    min_sliders: [range.end() as f64, 0.0, 0.0, 0.0],
                    ..Default::default()
                });
            }
            SettingKind::FloatRange {
                range,
                logarithmic,
                default_value,
            } => {
                parameter_manager.append_float(NumericParameter {
                    name: String::from(id.name()),
                    label: String::from(label),
                    default_values: [default_value as f64, 0.0, 0.0, 0.0],
                    min_sliders: [range.start() as f64, 0.0, 0.0, 0.0],
                    min_sliders: [range.end() as f64, 0.0, 0.0, 0.0],
                    ..Default::default()
                });
            }
            SettingKind::Boolean { default_value } => {
                parameter_manager.append_toggle(NumericParameter {
                    name: String::from(id.name()),
                    label: String::from(label),
                    default_values: [default_value as f64, 0.0, 0.0, 0.0],
                    ..Default::default()
                });
            }
            SettingKind::Group {
                children,
                default_value,
            } => {
                parameter_manager.append_header(StringParameter {
                    name: String::from(id.name()),
                    label: String::from(label),
                    ..Default::default()
                });
                for child in children {
                    self.register_setting_descriptor(&child, parameter_manager);
                }
            }
        }
    }

    fn update_setting(&mut self, inputs: &ParamInputs) {}
    fn get_all_setting_ids() -> Vec<SettingID> {
        SettingsList::new()
            .by_id
            .iter()
            .map(|bb| SettingID::from(bb.unwrap().as_ref()))
            .flatten()
            .collect()
    }

    fn find_setting_in_tree(
        setting_id: SettingID,
        descriptor: SettingDescriptor,
    ) -> Option<SettingDescriptor> {
        if descriptor.id == setting_id {
            return Some(descriptor);
        }

        if let Some(SettingKind::Group { children, .. }) = descriptor.kind {
            for child in children {
                if let Some(found) = Self::find_setting_in_tree(setting_id, child) {
                    return Some(found);
                }
            }
        }

        return None;
    }
    fn get_setting_desc_from_id(setting_id: SettingID) -> Option<SettingDescriptor> {
        let settings = SettingsList::new().settings.iter();

        for sett in settings {
            if let Some(found) = Self::find_setting_in_tree(setting_id, sett) {
                return Some(found);
            }
        }

        return None;
    }
}

fn get_setting_descriptor() -> SettingDescriptor {
    todo!();
}

impl OperatorParams for NtscTopParams {
    fn register(&mut self, parameter_manager: &mut ParameterManager) {
        let params = SettingsList::new().settings;

        for setting in params.iter() {
            self.register_setting_descriptor(setting, parameter_manager);
        }
    }

    fn update(&mut self, inputs: &ParamInputs) {
        inputs.type_id()
    }
}
*/
