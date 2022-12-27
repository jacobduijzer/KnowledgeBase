var settings = loadJsonContent('settings.json')

var solutionName = toLower(settings.solutionName)

resource swa_resource 'Microsoft.Web/staticSites@2021-01-15' = {
  name: 'swa-${solutionName}'
  location: settings.location
  tags: null
  properties: {}
  sku: {
      name: 'Standard' 
      size: 'Standard'
  }
}
