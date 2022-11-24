<?php
  http_response_code(201);
  header('Content-Type: application/json; charset=utf-8');

  $object = array(
    'resource' => array(
      'id' => 42,
      'name' => 'some-resource-name',
    )
  );
  echo json_encode($object);
?>