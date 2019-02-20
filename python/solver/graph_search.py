def graph_search(repository_package_list, initial_state, final_state):
    # install deps before installing package. uninstall deps after
    # uninstalling package

    # need to add anything which depends on package to list of
    # required uninstalls when adding something to required uninstalls

    # required uninstalls = final state uninstalls : stack of sets
    # assume nothing installed
    # for each install command
        # try install p:
        # add any (conflicts + deps) to list of required uninstalls stack
        # for each dep of p
            # set of tried_ids = {}
            # while possible pick an alternative q not in tried_ids with no deps on required uninstalls,
            #    (based on heuristic or randomly, e.g. number of deps or -number of conflicts)
                # add q.id to tried_ids
                # succes try install q
        # break;
    pass
