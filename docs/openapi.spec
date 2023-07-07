{
  "openapi": "3.0.0",
  "info": {
    "title": "Scroll Rollup Explorer",
    "version": "2.0"
  },
  "servers": [
    {
      "url": "http://localhost:5001/api"
    }
  ],
  "tags": [],
  "paths": {
    "/batch": {
      "get": {
        "parameters": [
          {
            "name": "index",
            "schema": {
              "type": "integer",
              "format": "int64"
            },
            "in": "query",
            "required": true,
            "deprecated": false,
            "explode": true
          }
        ],
        "responses": {
          "200": {
            "description": "",
            "content": {
              "application/json; charset=utf-8": {
                "schema": {
                  "$ref": "#/components/schemas/BatchResponse"
                }
              }
            }
          }
        }
      }
    },
    "/batches": {
      "get": {
        "parameters": [
          {
            "name": "page",
            "schema": {
              "type": "integer",
              "format": "uint64"
            },
            "in": "query",
            "required": false,
            "deprecated": false,
            "explode": true
          },
          {
            "name": "per_page",
            "schema": {
              "type": "integer",
              "format": "uint64"
            },
            "in": "query",
            "required": false,
            "deprecated": false,
            "explode": true
          }
        ],
        "responses": {
          "200": {
            "description": "",
            "content": {
              "application/json; charset=utf-8": {
                "schema": {
                  "$ref": "#/components/schemas/BatchesResponse"
                }
              }
            }
          }
        }
      }
    },
    "/batch_blocks": {
      "get": {
        "parameters": [
          {
            "name": "batch_index",
            "schema": {
              "type": "integer",
              "format": "int64"
            },
            "in": "query",
            "required": true,
            "deprecated": false,
            "explode": true
          }
        ],
        "responses": {
          "200": {
            "description": "",
            "content": {
              "application/json; charset=utf-8": {
                "schema": {
                  "$ref": "#/components/schemas/BlocksResponse"
                }
              }
            }
          }
        }
      }
    },
    "/chunks": {
      "get": {
        "parameters": [
          {
            "name": "batch_index",
            "schema": {
              "type": "integer",
              "format": "int64"
            },
            "in": "query",
            "required": true,
            "deprecated": false,
            "explode": true
          }
        ],
        "responses": {
          "200": {
            "description": "",
            "content": {
              "application/json; charset=utf-8": {
                "schema": {
                  "$ref": "#/components/schemas/ChunksResponse"
                }
              }
            }
          }
        }
      }
    },
    "/chunk_blocks": {
      "get": {
        "parameters": [
          {
            "name": "chunk_index",
            "schema": {
              "type": "integer",
              "format": "int64"
            },
            "in": "query",
            "required": true,
            "deprecated": false,
            "explode": true
          }
        ],
        "responses": {
          "200": {
            "description": "",
            "content": {
              "application/json; charset=utf-8": {
                "schema": {
                  "$ref": "#/components/schemas/BlocksResponse"
                }
              }
            }
          }
        }
      }
    },
    "/last_batch_indexes": {
      "get": {
        "responses": {
          "200": {
            "description": "",
            "content": {
              "application/json; charset=utf-8": {
                "schema": {
                  "$ref": "#/components/schemas/LastBatchIndexesResponse"
                }
              }
            }
          }
        }
      }
    },
    "/search": {
      "get": {
        "parameters": [
          {
            "name": "keyword",
            "schema": {
              "type": "string"
            },
            "in": "query",
            "required": true,
            "deprecated": false,
            "explode": true
          }
        ],
        "responses": {
          "200": {
            "description": "",
            "content": {
              "application/json; charset=utf-8": {
                "schema": {
                  "$ref": "#/components/schemas/SearchResponse"
                }
              }
            }
          }
        }
      }
    }
  },
  "components": {
    "schemas": {
      "Batch": {
        "type": "object",
        "required": [
          "hash",
          "index",
          "start_chunk_index",
          "end_chunk_index",
          "start_block_number",
          "end_block_number",
          "total_tx_num",
          "rollup_status",
          "created_at"
        ],
        "properties": {
          "hash": {
            "type": "string"
          },
          "index": {
            "type": "integer",
            "format": "int64"
          },
          "start_chunk_index": {
            "type": "integer",
            "format": "int64"
          },
          "end_chunk_index": {
            "type": "integer",
            "format": "int64"
          },
          "start_block_number": {
            "type": "integer",
            "format": "int64"
          },
          "end_block_number": {
            "type": "integer",
            "format": "int64"
          },
          "total_tx_num": {
            "type": "string",
            "format": "decimal"
          },
          "rollup_status": {
            "type": "string"
          },
          "commit_tx_hash": {
            "type": "string"
          },
          "finalize_tx_hash": {
            "type": "string"
          },
          "created_at": {
            "type": "string",
            "format": "decimal"
          },
          "committed_at": {
            "type": "string",
            "format": "decimal"
          },
          "finalized_at": {
            "type": "string",
            "format": "decimal"
          }
        }
      },
      "BatchResponse": {
        "type": "object",
        "properties": {
          "batch": {
            "$ref": "#/components/schemas/Batch"
          }
        }
      },
      "BatchesResponse": {
        "type": "object",
        "required": [
          "total",
          "batches"
        ],
        "properties": {
          "total": {
            "type": "integer",
            "format": "int64"
          },
          "batches": {
            "type": "array",
            "items": {
              "$ref": "#/components/schemas/Batch"
            }
          }
        }
      },
      "Block": {
        "type": "object",
        "required": [
          "number",
          "tx_num",
          "hash",
          "block_timestamp"
        ],
        "properties": {
          "number": {
            "type": "integer",
            "format": "int64"
          },
          "tx_num": {
            "type": "integer",
            "format": "int32"
          },
          "hash": {
            "type": "string"
          },
          "block_timestamp": {
            "type": "string",
            "format": "decimal"
          }
        }
      },
      "BlocksResponse": {
        "type": "object",
        "required": [
          "blocks"
        ],
        "properties": {
          "batch_index": {
            "type": "integer",
            "format": "int64"
          },
          "chunk_index": {
            "type": "integer",
            "format": "int64"
          },
          "blocks": {
            "type": "array",
            "items": {
              "$ref": "#/components/schemas/Block"
            }
          }
        }
      },
      "Chunk": {
        "type": "object",
        "required": [
          "hash",
          "index",
          "start_block_number",
          "end_block_number",
          "total_tx_num",
          "created_at"
        ],
        "properties": {
          "hash": {
            "type": "string"
          },
          "index": {
            "type": "integer",
            "format": "int64"
          },
          "start_block_number": {
            "type": "integer",
            "format": "int64"
          },
          "end_block_number": {
            "type": "integer",
            "format": "int64"
          },
          "total_tx_num": {
            "type": "integer",
            "format": "int64"
          },
          "created_at": {
            "type": "string",
            "format": "decimal"
          }
        }
      },
      "ChunksResponse": {
        "type": "object",
        "required": [
          "batch_index",
          "chunks"
        ],
        "properties": {
          "batch_index": {
            "type": "integer",
            "format": "int64"
          },
          "chunks": {
            "type": "array",
            "items": {
              "$ref": "#/components/schemas/Chunk"
            }
          }
        }
      },
      "LastBatchIndexesResponse": {
        "type": "object",
        "required": [
          "all_index",
          "committed_index",
          "finalized_index"
        ],
        "properties": {
          "all_index": {
            "type": "integer",
            "format": "int64"
          },
          "committed_index": {
            "type": "integer",
            "format": "int64"
          },
          "finalized_index": {
            "type": "integer",
            "format": "int64"
          }
        }
      },
      "SearchResponse": {
        "type": "object",
        "required": [
          "batch_index"
        ],
        "properties": {
          "batch_index": {
            "type": "integer",
            "format": "int64"
          }
        }
      }
    }
  }
}