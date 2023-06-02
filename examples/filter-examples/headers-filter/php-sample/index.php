<h1> HTTP Headers: </h1><ul>
<?php

if (!function_exists('getallheaders')) {
    # Based on https://stackoverflow.com/a/20164575/685637
    function getallheaders()
    {
        $headers = [];
        foreach ($_SERVER as $name => $value) {
            if (substr($name, 0, 5) == 'HTTP_') {
                $headers[str_replace(' ', '-', ucwords(strtolower(str_replace('_', ' ', substr($name, 5)))))] = $value;
            }
        }
        return $headers;
    }
}

$headers = getallheaders();
foreach ($headers as $header => $value) {
    echo "<li><pre>";
    echo "$header : $value";
    echo "</pre></li>\n";
}
?>
</ul>

<?php

if (isset($_GET["with-env"])) {
    echo "<h1> Environment variables: </h1><ul>";
    $env_variables =getenv();
    foreach ($env_variables as $var => $value) {
        echo "<li><pre>";
        echo "$var=$value";
        echo "</pre></li>\n";
    }
    echo "</ul>";
}
?>
