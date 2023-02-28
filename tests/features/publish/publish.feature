Feature: PubNub publish feature

  Background:
    Given the demo keyset

  @contract=publishContractScript
  Scenario: Simple publish
    When I publish a message
    Then I get a timetoken
