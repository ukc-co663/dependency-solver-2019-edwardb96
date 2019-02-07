def repository_to_dictionary(package_list):
    package_dictionary = {}
    for package in package_list:
        if package.name in package_dictionary:
            package_dictionary[package.name].append(package)
        else:
            package_dictionary[package.name] = [package]
    return package_dictionary
