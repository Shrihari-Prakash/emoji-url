<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>Rust URL Shortener</title>
        <link rel="stylesheet" href="static/style.css">
    </head>
    <body>
        <h2>Rust URL Shortener</h2>
        <h3>{{result}}</h3>
        <form action="/shorten" method="POST">
            <input type="text" name="url" style="width: 35em;" />
            <input type="submit" value="Shorten!" />
        </form>
        <br><p>{{url_count}} URLs shortened</p>
    </body>
</html>