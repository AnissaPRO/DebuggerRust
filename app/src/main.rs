use slint::{ComponentHandle, VecModel};
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let raw_data = core_logger::parse_log_file("logs.log");

    let tous_les_logs: Vec<LogEntry> = raw_data
        .into_iter()
        .map(|item| LogEntry {
            status: slint::SharedString::from(item.level),
            date: slint::SharedString::from(item.date),
            containerId: slint::SharedString::from(item.container_id),
            message: slint::SharedString::from(item.message),
            temps_reponse: slint::SharedString::from("0ms"),
        })
        .collect();

    let model = Rc::new(VecModel::from(tous_les_logs.clone()));
    ui.set_logs(model.into());

    let logs_reference = tous_les_logs;
    let ui_handle = ui.as_weak();

    ui.on_filtrer_par_niveau(move |niveau| {
        if let Some(ui) = ui_handle.upgrade() {
            let logs_filtres: Vec<LogEntry> = logs_reference
                .iter()
                .filter(|log| {
                    let log_level = log.status.to_lowercase();
                    match niveau.as_str() {
                        "all" => true,
                        "error" => log_level.contains("error") || log_level.contains("err"),
                        "warn" => log_level.contains("warn") || log_level.contains("warning"),
                        "info" => log_level.contains("info"),
                        _ => true,
                    }
                })
                .cloned()
                .collect();

            let nouveau_model = Rc::new(VecModel::from(logs_filtres));
            ui.set_logs(nouveau_model.into());
        }
    });

    ui.run()
}
