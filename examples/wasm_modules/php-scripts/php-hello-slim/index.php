<html>
<body>
<?php

print "<h1>Hello from PHP-slim on Wasm!</h1>";

$date = getdate();

$message = "Today, ";
$message .= $date['weekday'] . ", ";

$message .= $date['year'] . "-";
$message .= $date['mon'] . "-";
$message .= $date['mday'];

$message .= ", at ";
$message .= $date['hours'] . ":";
$message .= $date['minutes'] . ":";
$message .= $date['seconds'];

$message .= " we greet you with this message!";
print $message;

print "<h1>Output from phpinfo():</h1>";
phpinfo();
?>
</body>
</html>
