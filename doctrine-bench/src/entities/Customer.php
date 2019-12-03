<?php
namespace entities;

use Doctrine\ORM\Mapping as ORM;

/**
 * entities\Customer
 *
 * @Entity
 * @Table(name="customer")
 */
class Customer
{
    /**
     * @Id
     * @Column(type="integer")
     */
    private $customer_id;
    /**
     * @Column(type="smallint")
     */
    private $store_id;
    /**
     * @Column(type="string")
     */
    private $first_name;
    /**
     * @Column(type="string")
     */
    private $last_name;
    /**
     * @Column(type="string")
     */
    private $email;
    /**
     * @Column(type="smallint")
     */
    private $address_id;
    /**
     * @Column(type="boolean")
     */
    private $activebool;
    /**
     * @Column(type="date")
     */
    private $create_date;
    /**
     * @Column(type="datetime")
     */
    private $last_update;
    /**
     * @Column(type="integer")
     */
    private $active;
}