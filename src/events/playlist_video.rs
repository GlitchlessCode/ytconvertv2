pub enum PlaylistVideoSettingsEvent {
    SetInclusion(bool),
    StartEdit,
    EndEdit(String),
}
