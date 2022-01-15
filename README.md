# Uniform

When developing web applications it can be challenging to maintain a consistent, accessible and secure UI. Uniform aims to make this easier. It aims to remove several degrees of freedom from developers so that they can focus on developing the core logic of the application. This means that all views genrated by Uniform have the same look and feel. All interactions follow a similar pattern and as such a user that have used one uniform application should be quite familiar with another.

This comes with several important downsides:

 - Loss of flexibility in design
 - Loss of custom UX patterns
 - Loss of extensibility

As such the Uniform tool is not suitable for all types of web applications. In particular applications with a rich UI that requires a lot of client side logic should likely not use Uniform. Applications where the core logic lies server side are much more likely to be a good fit for the approach Uniform is taking.

## Goals
 - Consistent UI with minimal amount of surprises
 - Accessiblity
 - Internationalization and localization support
 - Minimal amount of client side code
 - Minimal cookies
 - OpenID Connect based or SAML based authentication?
 - Capability based security
 - Multi-backend support for different languages

## Approach
Working with uniform starts with creating a specification of the entities that a user will interact with. This specification defines what properties each entity has along with any constraints that exist on those properties. In addition each entity has an overview definition for when it should be shown in an abreviated version.

Once the specification is complete the uniform tool takes it as input and genreates localisation files (based on the desired langugages that should be supported) and a config file for non-locale dependent general properties. After specifying translations and the general properties uniform can be used to generate a library that encapsulate the generation of the web pages used in the application. 

At this point the developers can add the library as a dependency in their application and begin implementing the logic needed to fetch the specified entities.