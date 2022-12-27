//
// Creates a resource group with the name 'rg-solutionname'
//

var solutionName = 'knowledgebase'
targetScope = 'subscription'

@description('The location.')
param location string = 'westeurope'

resource rg 'Microsoft.Resources/resourceGroups@2021-04-01' = {
  name: 'rg--${solutionName}'
  location: location
}
