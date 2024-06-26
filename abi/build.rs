use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .type_attribute("reservation.ReservationStatus", "#[derive(sqlx::Type)]")
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    Command::new("cargo").arg("fmt").output().unwrap();

    println!("cargo:rerun-if-changed=protos/reservation.proto");
}
