Feature: Dependency Mock
  Background:
    * def outs = 
    """
    {
      "myTest": true
    }
    """

  Scenario: pathMatches('/can-do/{id}') && methodIs('get')
    * def responseStatus = 200
    * def response = {}
    * response.can_do = outs[pathParams.id] || false
