pub fn is_building_on_render_dot_com() -> bool {
    let render_env_var = std::env::var("RENDER").unwrap_or_else(|_| "false".to_string());
    render_env_var.to_lowercase() == "true"
}
