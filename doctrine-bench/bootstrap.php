<?php

use Doctrine\ORM\Query\ResultSetMapping;
use Doctrine\ORM\Tools\Setup;
use Doctrine\ORM\EntityManager;
use entities\Customer;

require_once "vendor/autoload.php";

// Create a simple "default" Doctrine ORM configuration for Annotations
$isDevMode = true;
$proxyDir = null;
$cache = null;
$useSimpleAnnotationReader = true;
$config = Setup::createAnnotationMetadataConfiguration(array(__DIR__ . "/entities"), $isDevMode, $proxyDir, $cache, $useSimpleAnnotationReader);

// database configuration parameters
$conn = array(
    'url' => 'postgresql://tg:@localhost/postgres'
);

// obtaining the entity manager
$entityManager = EntityManager::create($conn, $config);


$rsm = new ResultSetMapping();
$rsm->addEntityResult('entities\Customer', "c");

$query = $entityManager->createNativeQuery("select c.customer_id, c.store_id, c.first_name, c.last_name, c.email, c.address_id, c.activebool, c.create_date, c.last_update, c.active
 from customer as c", $rsm);
//$query->setParameter(1, 20);

$customers = $query->getResult();
echo count($customers);
foreach ($customers as $c) {
    print_r($c);
}