<html>
<body>
<h1>Hello from php on wasm</h1>
<?php
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
?>
</body>
</html>
