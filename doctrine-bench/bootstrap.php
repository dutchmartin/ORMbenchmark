<?php

use Doctrine\ORM\Query\ResultSetMapping;
use Doctrine\ORM\Tools\Setup;
use Doctrine\ORM\EntityManager;
use entities\InsertableCustomer;

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
$rsm->addFieldResult('c','customer_id','customer_id');
$rsm->addFieldResult('c','store_id','store_id');
$rsm->addFieldResult('c','first_name','first_name');
$rsm->addFieldResult('c','last_name','last_name');
$rsm->addFieldResult('c','email','email');
$rsm->addFieldResult('c','address_id','address_id');
$rsm->addFieldResult('c','activebool','activebool');
$rsm->addFieldResult('c','create_date','create_date');
$rsm->addFieldResult('c','last_update','last_update');
$rsm->addFieldResult('c','active','active');




$query = $entityManager->createNativeQuery("select c.customer_id, c.store_id, c.first_name, c.last_name, c.email, c.address_id, c.activebool, c.create_date, c.last_update, c.active
 from customer as c  order by c.customer_id ASC limit ?", $rsm);
$query->setParameter(1, 20);

$customers = $query->getResult();
echo count($customers);
foreach ($customers as $c) {
    var_dump($c);
}

$new_customers = array(
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ),
    new InsertableCustomer(
        null,
        1,
        "john",
        "doe",
        "jhon@doe.com",
        2,
        1,
        1
        ));

foreach ($new_customers as $new_customer) {
    $entityManager->persist($new_customer);
}
$entityManager->flush();
