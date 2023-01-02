var settings = loadJsonContent('settings.json')

var solutionName = toLower(settings.solutionName)
var customDomain1 = settings.customDomain1
var customDomain2 = settings.customDomain2

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

resource staticwebApplicationDomain1 'Microsoft.Web/staticSites/customDomains@2022-03-01' = {
  name: '${customDomain1}'
  parent: swa_resource
}

resource staticwebApplicationDomain2 'Microsoft.Web/staticSites/customDomains@2022-03-01' = {
  name: '${customDomain2}'
  parent: swa_resource
}
