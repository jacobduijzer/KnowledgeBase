var settings = loadJsonContent('settings.json')

var solutionName = toLower(settings.solutionName)

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

output deployment_token string = listSecrets(swa_resource.id, swa_resource.apiVersion).properties.apiKey

