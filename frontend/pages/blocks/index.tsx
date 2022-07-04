import type { NextPage } from "next";
import React from "react";
import { Input } from "antd";
import Table from "./Table";
import { useL1Tps, useL2Tps } from "hooks";

const Blocks: NextPage = () => {
  const { l1Tps } = useL1Tps();
  const { l2Tps } = useL2Tps();
  return (
    <div>
      <main>
        <div className="">
          <div className="wrapper flex items-center  h-120px justify-between">
            <div>
              <p className="text-36px  leading-44px font-700 mb-15px">Blocks</p>
            </div>
            <div className="flex">
              <div className="text-center mr-60px">
                <h2 className="text-32px">{l1Tps?.tps ?? 0}</h2>
                <p className="text-[#9ba4b3]">L1 TPS</p>
              </div>
              <div className="text-center ">
                <h2 className="text-32px">{l2Tps?.tps ?? 0} </h2>
                <p className="text-[#9ba4b3]">L2 TPS</p>
              </div>
            </div>
          </div>
        </div>
        <div className="bg-[#f6f7f8]">
          <div className="wrapper py-60px">
            <Table />
          </div>
        </div>
      </main>
    </div>
  );
};

export default Blocks;
