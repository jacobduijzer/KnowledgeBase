var settings = loadJsonContent('settings.json')

var solutionName = toLower(settings.solutionName)
var customDomain = settings.customDomain

resource swa_resource 'Microsoft.Web/staticSites@2021-01-15' = {
  name: 'swa-${solutionName}'
  location: settings.location
  tags: null
  properties: {}
  sku: {
      name: 'Free' 
      size: 'Free'
  }
}

resource staticwebApplicationDomain 'Microsoft.Web/staticSites/customDomains@2022-03-01' = {
  name: '${customDomain}'
  parent: swa_resource
}
