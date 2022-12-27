//
// Creates a resource group with the name 'rg-solutionname'
//
targetScope = 'subscription'

var settings = loadJsonContent('settings.json')

var solutionName = toLower(settings.solutionName)

resource rg 'Microsoft.Resources/resourceGroups@2021-04-01' = {
  name: 'rg-${solutionName}'
  location: settings.location
}
