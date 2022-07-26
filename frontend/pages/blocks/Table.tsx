import { Table, Tag } from "antd";
import type { ColumnsType, TablePaginationConfig } from "antd/lib/table";
import type { FilterValue, SorterResult } from "antd/lib/table/interface";
import React, { useEffect, useState } from "react";
import { BLOCK_STATUS, EXPLORER } from "constants/index";
import Link from "next/link";
import moment from "moment";
import clsx from "clsx";

interface DataType {
  block_height: number;
  header_hash: string;
  l1_tx_hash: string;
  status: string;
  block_timestamp: string;
  tx_num: number;
  finalize_tx_hash: string;
  rollup_tx_hash: string;
}

interface Params {
  pagination?: TablePaginationConfig;
  sorter?: SorterResult<any> | SorterResult<any>[];
  total?: number;
  sortField?: string;
  sortOrder?: string;
}

const App: React.FC = () => {
  const [data, setData] = useState();
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState<TablePaginationConfig>({
    current: 1,
    pageSize: 10,
  });
  const columns: ColumnsType<DataType> = [
    {
      title: "Height",
      dataIndex: "block_height",
      key: "block_height",
      // render: (text) => <Link href={"#"}>{text}</Link>,
    },
    {
      title: "Header hash",
      dataIndex: "header_hash",
      key: "header_hash",
      render: (hash) => (
        <Link href={`${EXPLORER.L2_EXPLORER}/block/${hash}`}>{hash}</Link>
      ),
    },
    {
      title: "Status",
      key: "status",
      dataIndex: "status",
      render: (_, { status, finalize_tx_hash, rollup_tx_hash }) => {
        let color, targetUrl: string;

        switch (status) {
          case BLOCK_STATUS.FINALIZED:
            color = "success";
            targetUrl = `${EXPLORER.L1_EXPLORER}/tx/${finalize_tx_hash}`;
            break;
          case BLOCK_STATUS.PRECOMMITTED:
            color = "warning";
            break;
          case BLOCK_STATUS.COMMITTED:
            targetUrl = `${EXPLORER.L1_EXPLORER}/tx/${rollup_tx_hash}`;
            color = "processing";
            break;
          default:
            color = "default";
            break;
        }

        return (
          <>
            <Tag
              color={color}
              className={
                BLOCK_STATUS.PRECOMMITTED === status ? "" : "cursor-pointer"
              }
              onClick={() => handleChange(targetUrl)}
            >
              {status.toUpperCase()}
            </Tag>
          </>
        );
      },
    },
    {
      title: "Time",
      dataIndex: "block_timestamp",
      key: "block_timestamp",
      render: (_, { block_timestamp }) => (
        <>
          {moment(new Date(+block_timestamp * 1000)).format(
            "MMM. D, yyyy hh:mm a"
          )}
        </>
      ),
    },
    {
      title: "Txn",
      dataIndex: "tx_num",
      key: "tx_num",
    },
  ];

  const fetchData = (pagination: TablePaginationConfig = {}) => {
    setLoading(true);
    fetch(
      `${process.env.NEXT_PUBLIC_BASE_API_URL}/l2_blocks?page=${pagination.current}&per_page=${pagination.pageSize}`
    )
      .then((res) => res.json())
      .then(({ blocks, total }) => {
        setData(blocks);
        setLoading(false);
        setPagination({
          ...pagination,
          total,
        });
      });
  };

  useEffect(() => {
    fetchData(pagination);
  }, []);

  const handleTableChange = (newPagination: TablePaginationConfig) => {
    fetchData(newPagination);
  };

  const handleChange = (targetUrl: string) => {
    if (targetUrl) {
      window.location.href = targetUrl;
    }
  };

  return (
    <Table
      columns={columns}
      rowKey={(record) => record.block_height}
      dataSource={data}
      pagination={pagination}
      loading={loading}
      onChange={handleTableChange}
    />
  );
};

export default App;
