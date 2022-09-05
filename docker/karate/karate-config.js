function fn() {   
  var env = karate.env; // get java system property 'karate.env'
  karate.log('karate.env system property was:', env);
  // Can optionally change this config based on the ENV if you need to invoke a different
  // URL based on running in a test environment vs a docker environment, etc
  return {
    baseUrl: 'http://service:8080/'
  }
}
