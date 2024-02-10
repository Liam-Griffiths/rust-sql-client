struct MySqlViewerApp {
    connection_details: ConnectionDetails,
    current_query: String,
    query_result: Vec<Vec<String>>, // Simplified for example purposes
    table_list: Vec<String>,
}

struct ConnectionDetails {
    host: String,
    username: String,
    password: String,
    database: String,
}

impl Default for ConnectionDetails {
    fn default() -> Self {
        Self {
            host: String::new(),
            username: String::new(),
            password: String::new(),
            database: String::new(),
        }
    }
}

impl Default for MySqlViewerApp {
    fn default() -> Self {
        Self {
            connection_details: ConnectionDetails::default(),
            current_query: String::new(),
            query_result: Vec::new(),
            table_list: Vec::new(),
        }
    }
}

impl eframe::App for MySqlViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("table_list").show(ctx, |ui| {
            ui.heading("Tables");
            for table in &self.table_list {
                if ui.button(table).clicked() {
                    // Optional: Load table data on click
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Host:");
                ui.text_edit_singleline(&mut self.connection_details.host);
            });
            ui.horizontal(|ui| {
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.connection_details.username);
            });
            ui.horizontal(|ui| {
                ui.label("Password:");
                ui.text_edit_singleline(&mut self.connection_details.password);
            });
            ui.horizontal(|ui| {
                ui.label("Database:");
                ui.text_edit_singleline(&mut self.connection_details.database);
            });

            if ui.button("Connect").clicked() {
                self.connect_to_database();
            }

            ui.separator();

            ui.text_edit_singleline(&mut self.current_query);
            if ui.button("Execute").clicked() {
                self.execute_query();
            }

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for row in &self.query_result {
                    ui.horizontal(|ui| {
                        for cell in row {
                            ui.label(cell);
                        }
                    });
                }
            });
        });
    }
}

use mysql::prelude::*;
use mysql::from_value_opt;
use mysql::{Pool, OptsBuilder};

impl MySqlViewerApp {
    fn connect_to_database(&mut self) {
        let opts: OptsBuilder = OptsBuilder::new()
            .ip_or_hostname(Some(&self.connection_details.host.clone()))
            .user(Some(&self.connection_details.username.clone()))
            .pass(Some(&self.connection_details.password.clone()))
            .db_name(Some(&self.connection_details.database.clone()))
            .into(); // This correctly infers Opts, no need to explicitly type

        if let Ok(_pool) = Pool::new(opts) {
            println!("Successfully connected to the database.");
            // Successfully connected to the database
            // You can now use `pool` to interact with your database
        } else {
            println!("Failed to connect to the database.");
            // Handle connection failure
        }
    }

    fn execute_query(&mut self) {
        let opts: mysql::Opts = OptsBuilder::new()
            .ip_or_hostname(Some(&self.connection_details.host.clone()))
            .user(Some(&self.connection_details.username.clone()))
            .pass(Some(&self.connection_details.password.clone()))
            .db_name(Some(&self.connection_details.database.clone()))
            .into();
        let pool = Pool::new(opts).expect("Failed to create pool.");

        self.query_result.clear(); // Clear previous query results first

        // Scope for the connection
        {
            let mut conn = pool.get_conn().expect("Failed to get connection.");
            let query = &self.current_query;

            // Execute the query and process the results within the same scope
            match conn.query_iter(query) {
                Ok(result) => {
                    for row_result in result {
                        let row = row_result.expect("Failed to read row.");
                        let row_values: Vec<String> = row.unwrap().into_iter().map(|value| {
                            from_value_opt::<String>(value).unwrap_or_else(|_| "NULL".to_string())
                        }).collect();
                        self.query_result.push(row_values);
                    }
                    println!("Query executed successfully.");
                },
                Err(e) => {
                    println!("Failed to execute query: {}", e);
                }
            };
        } // `conn` goes out of scope here, after all its uses are done

        // Any code here is safe as it doesn't depend on `conn` being alive
    }

}


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "MySql Viewer",
        options,
        Box::new(|_cc| Box::new(MySqlViewerApp::default())),
    ).expect("TODO: panic message");
}
