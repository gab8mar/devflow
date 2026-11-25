mod commands;
mod state;

use std::sync::Arc;
use std::time::Duration;

use devtime_application::ports::{Clock, TimeEntries};
use devtime_application::use_cases::timer::{self, RECOVERY_STALE_AFTER_MS};
use devtime_domain::EntryStatus;
use devtime_infrastructure::{Database, SystemClock};
use state::AppState;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_local_data_dir()?;
            let db = Arc::new(Database::open(data_dir.join("devtime.db"))?);
            let clock: Arc<dyn Clock> = Arc::new(SystemClock);
            let started_at_ms = clock.now_ms();

            db.ensure_default_workspace(started_at_ms)?;
            if timer::recover_stale(db.as_ref(), started_at_ms, RECOVERY_STALE_AFTER_MS)? {
                eprintln!("DevTime: entrada huérfana pausada en el último latido");
            }

            spawn_heartbeat(Arc::clone(&db), Arc::clone(&clock));

            app.manage(AppState {
                db,
                clock,
                started_at_ms,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::health::health_check,
            commands::preferences::get_preference,
            commands::preferences::set_preference,
            commands::timer::get_timer_snapshot,
            commands::timer::start_task,
            commands::timer::pause_current,
            commands::timer::resume_current,
            commands::timer::close_current,
            commands::timer::get_today_totals,
            commands::tasks::list_tasks,
            commands::tasks::create_task
        ])
        .run(tauri::generate_context!())
        .expect("no se pudo iniciar DevTime");
}

fn spawn_heartbeat(db: Arc<Database>, clock: Arc<dyn Clock>) {
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(60));
        if let Ok(Some(mut active)) = db.find_current() {
            if active.entry.status == EntryStatus::Running {
                active.entry.heartbeat(clock.now_ms());
                let _ = db.save(&active.entry);
            }
        }
    });
}
