pub const LOGIN_FORM: &str = r#"<!doctype html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Margaret — Log in</title>
  </head>
  <body>
    <h1>Log in</h1>
    <form method="post" action="/login">
      <label>
        Username
        <input name="username" autocomplete="username">
      </label>
      <label>
        Password
        <input name="password" type="password" autocomplete="current-password">
      </label>
      <button type="submit">Log in</button>
    </form>
  </body>
</html>
"#;
