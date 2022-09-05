Feature: Verify Authorization
  Background:
    * url baseUrl

  Scenario: Can request myTest
    Given path 'myTest'
    When method GET
    Then status 200
    And match response == '200'

  Scenario: Cant request notMyTest
    Given path 'notMyTest'
    When method GET
    Then status 401
