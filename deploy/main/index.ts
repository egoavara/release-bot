import * as pulumi from "@pulumi/pulumi";
import * as k8s from "@pulumi/kubernetes";
import * as rand from "@pulumi/random";
import * as eck from "eck-stack";

const namespace = new k8s.core.v1.Namespace("release-bot", {});

const elasticsearch = new eck.elasticsearch.v1.Elasticsearch("elastic-search", {
    metadata: {
        namespace: namespace.metadata.name,
    },
    spec: {
        version: "8.15.1",
        http: {
            tls: {
                selfSignedCertificate: { disabled: true }
            }
        },
        nodeSets: [
            {
                name: "default",
                count: 1,
                config: {
                    "node.store.allow_mmap": false,
                    "xpack.security.authc.anonymous": {
                        authz_exception: false,
                        roles: "superuser",
                        username: "anonymous",
                    },
                },
                volumeClaimTemplates: [
                    {
                        metadata: {
                            name: "elasticsearch-data"
                        },
                        spec: {
                            accessModes: ["ReadWriteOnce"],
                            resources: {
                                requests: {
                                    storage: "5Gi"
                                }
                            },
                            storageClassName: "local-path"
                        }
                    }
                ]
            }
        ]
    }
})

const kibana = new eck.kibana.v1.Kibana("kibana", {
    metadata: {
        namespace: namespace.metadata.name,
    },
    spec: {
        version: "8.15.1",
        count: 1,
        elasticsearchRef: {
            name: elasticsearch.metadata.name
        }
    }
})

export const PostgresAdminPassword = new rand.RandomPassword("postgres-admin-password", { length: 24, special: false })
export const PostgresDefaultPassword = new rand.RandomPassword("postgres-default-password", { length: 24, special: false })

const postgresSecret = new k8s.core.v1.Secret("postgres-auth", {
    metadata: {
        namespace: namespace.metadata.name,
    },
    stringData: {
        "default-password": PostgresDefaultPassword.result,
        "default-username": "default",
        "postgres-password": PostgresAdminPassword.result,
    }
})

const postgres = new k8s.helm.v3.Release("postgres", {
    chart: "oci://registry-1.docker.io/bitnamicharts/postgresql",
    version: "16.4.3",
    namespace: namespace.metadata.name,
    values: {
        global: {
            postgresql: {
                auth: {
                    existingSecret: postgresSecret.metadata.name
                }
            }
        },
        primary: {
            networkPolicy: {
                enabled: false,
            },
            resources: {
                limits: {
                    cpu: "1000m",
                    memory: "1000Mi"
                },
                requests: {
                    cpu: "1000m",
                    memory: "1000Mi"
                }
            }
        }
    }
})