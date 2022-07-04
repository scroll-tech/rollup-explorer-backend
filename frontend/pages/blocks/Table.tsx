import { Table, Tag } from "antd";
import type { ColumnsType, TablePaginationConfig } from "antd/lib/table";
import type { FilterValue, SorterResult } from "antd/lib/table/interface";
import React, { useEffect, useState } from "react";
import { BLOCK_STATUS } from "constants/index";
import Link from "next/link";
import moment from "moment";

interface DataType {
  block_height: number;
  header_hash: string;
  l1_tx_hash: string;
  status: string;
  timestamp: string;
  tx_num: number;
}

interface Params {
  pagination?: TablePaginationConfig;
  sorter?: SorterResult<any> | SorterResult<any>[];
  total?: number;
  sortField?: string;
  sortOrder?: string;
}

const columns: ColumnsType<DataType> = [
  {
    title: "Height",
    dataIndex: "block_height",
    key: "block_height",
    render: (text) => <Link href={"#"}>{text}</Link>,
  },
  {
    title: "Header hash",
    dataIndex: "header_hash",
    key: "header_hash",
  },
  {
    title: "Status",
    key: "status",
    dataIndex: "status",
    render: (_, { status }) => {
      let color;
      switch (status) {
        case BLOCK_STATUS.VERIFIED:
          color = "success";
          break;
        case BLOCK_STATUS.UNCOMMITTED:
          color = "warning";
          break;
        case BLOCK_STATUS.COMMITTED:
          color = "processing";
          break;
        default:
          color = "default";
          break;
      }

      return (
        <>
          <Tag color={color}>{status.toUpperCase()}</Tag>
        </>
      );
    },
  },
  {
    title: "Time",
    dataIndex: "timestamp",
    key: "timestamp",
    render: (_, { timestamp }) => (
      <>{moment(new Date(+timestamp)).format("MMM. D, YYYY	hh:mm")}</>
    ),
  },
  {
    title: "Txn",
    dataIndex: "tx_num",
    key: "tx_num",
  },
];

const App: React.FC = () => {
  const [data, setData] = useState();
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState<TablePaginationConfig>({
    current: 1,
    pageSize: 10,
  });

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
