// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Diese Dublette fuehrt **keine** Kettenadoption mehr (Befund F4, P1d-A).
//!
//! Der frueher hier geprueft Fall ("zwei Nodes syncen Kette") ist mit
//! `sync_pull` entfernt worden. Kanonisch ist der Pfad in `components/node`:
//! `atc_node::gossip::sync_pull(.., validator)` -> `Chain::adopt_verified`.
//! Ein Test, der eine ungeprueste Adoption nachweisen wuerde, waere ein Test
//! genau des entfernten Fehlers.

#[test]
fn keine_adoption_im_modul() {
    let quelle = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/gossip.rs"))
        .expect("gossip.rs lesbar");
    for verboten in ["pub fn sync_pull", "= kandidat", "Chain::from_blocks"] {
        assert!(
            !quelle.contains(verboten),
            "die Parallelkopie darf keine Adoption anbieten (gefunden: {verboten})"
        );
    }
}
