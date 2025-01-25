// *** WARNING: this file was generated by crd2pulumi. ***
// *** Do not edit by hand unless you're certain you know what you are doing! ***

import * as pulumi from "@pulumi/pulumi";
import * as utilities from "../../utilities";

// Export members:
export { StackConfigPolicyArgs } from "./stackConfigPolicy";
export type StackConfigPolicy = import("./stackConfigPolicy").StackConfigPolicy;
export const StackConfigPolicy: typeof import("./stackConfigPolicy").StackConfigPolicy = null as any;
utilities.lazyLoad(exports, ["StackConfigPolicy"], () => require("./stackConfigPolicy"));

export { StackConfigPolicyListArgs } from "./stackConfigPolicyList";
export type StackConfigPolicyList = import("./stackConfigPolicyList").StackConfigPolicyList;
export const StackConfigPolicyList: typeof import("./stackConfigPolicyList").StackConfigPolicyList = null as any;
utilities.lazyLoad(exports, ["StackConfigPolicyList"], () => require("./stackConfigPolicyList"));

export { StackConfigPolicyPatchArgs } from "./stackConfigPolicyPatch";
export type StackConfigPolicyPatch = import("./stackConfigPolicyPatch").StackConfigPolicyPatch;
export const StackConfigPolicyPatch: typeof import("./stackConfigPolicyPatch").StackConfigPolicyPatch = null as any;
utilities.lazyLoad(exports, ["StackConfigPolicyPatch"], () => require("./stackConfigPolicyPatch"));


const _module = {
    version: utilities.getVersion(),
    construct: (name: string, type: string, urn: string): pulumi.Resource => {
        switch (type) {
            case "kubernetes:stackconfigpolicy.k8s.elastic.co/v1alpha1:StackConfigPolicy":
                return new StackConfigPolicy(name, <any>undefined, { urn })
            case "kubernetes:stackconfigpolicy.k8s.elastic.co/v1alpha1:StackConfigPolicyList":
                return new StackConfigPolicyList(name, <any>undefined, { urn })
            case "kubernetes:stackconfigpolicy.k8s.elastic.co/v1alpha1:StackConfigPolicyPatch":
                return new StackConfigPolicyPatch(name, <any>undefined, { urn })
            default:
                throw new Error(`unknown resource type ${type}`);
        }
    },
};
pulumi.runtime.registerResourceModule("crds", "stackconfigpolicy.k8s.elastic.co/v1alpha1", _module)
