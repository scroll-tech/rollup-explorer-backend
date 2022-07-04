import type { NextPage } from "next";
import React, { useEffect, useState } from "react";
import { Input } from "antd";

const { Search } = Input;

const Footer: NextPage = () => {
  return (
    <div className="footer py-100px bg-[#3f3238]">
      <div className="wrapper flex  flex-col md:(flex-row)">
        <div className="flex mb-30px flex-wrap-reverse md:(mb-0)">
          <ul className="link-container mr-100px mt-20px md:(mt-0)">
            <li>About</li>
            <li>
              <a href="https://scroll.io/blog">Blog</a>
            </li>
            <li>
              <a href="https://scroll.io/team">Team</a>
            </li>
          </ul>
          <ul className="link-container mr-100px">
            <li>Community</li>
            <li>
              <a
                href="https://jobs.lever.co/ScrollFoundation"
                rel="noreferrer"
                target="_blank"
              >
                Hiring
              </a>
            </li>
          </ul>
          <ul className="link-container">
            <li>Contact Us</li>
            <li className="text-white">
              Email:
              <a className="hover:underline" href="mailto:hi@scroll.io">
                hi@scroll.io
              </a>
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
};

export default Footer;
