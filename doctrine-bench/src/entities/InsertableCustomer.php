<?php

namespace entities;

use Doctrine\ORM\Mapping as ORM;

/**
 * entities\Customer
 *
 * @Entity
 * @Table(name="customer")
 */
class InsertableCustomer
{
    /**
     * @Id
     * @Column(type="integer")
     * @GeneratedValue
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
     * @Column(type="integer")
     */
    private $active;

    public function __construct($customer_id, $store_id, $first_name, $last_name, $email, $address_id, $activebool, $active)
    {
        $this->customer_id = $customer_id;
        $this->store_id = $store_id;
        $this->first_name = $first_name;
        $this->last_name = $last_name;
        $this->email = $email;
        $this->address_id = $address_id;
        $this->activebool = $activebool;
        $this->active = $active;
    }
}
