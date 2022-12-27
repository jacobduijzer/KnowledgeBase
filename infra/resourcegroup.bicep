//
// Creates a resource group with the name 'rg-<environment>-solutionname'
//

var solutionName = 'dataprocessor'
targetScope = 'subscription'

@description('The name of the environment. This must be test, accept or prod.')
@allowed([
  'test'
  'accept'
  'prod'
])
param environmentName string = 'test'

@description('The location.')
param location string = 'westeurope'

var baseName = '${environmentName}-${solutionName}'

resource rg 'Microsoft.Resources/resourceGroups@2021-04-01' = {
  name: 'rg-${environmentName}-${solutionName}'
  location: location
}
