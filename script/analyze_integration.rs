// src/main.rs (ajout route)
.route("/dashboard/stats", get(handlers::dashboard::get_dashboard_stats)
    .layer(middleware::from_fn_with_state(pool.clone(), auth_middleware)))
