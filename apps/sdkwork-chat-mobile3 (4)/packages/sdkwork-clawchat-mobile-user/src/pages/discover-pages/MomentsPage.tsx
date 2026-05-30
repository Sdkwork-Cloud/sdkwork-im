import React, { useState, useEffect } from "react";
import { PageLayout } from "../../components/SettingsCommons";
import { Camera, Heart, MessageCircle, Send } from "lucide-react";
import { MomentService, type Moment } from "../../services/MomentService";
import { Avatar, cn } from "@sdkwork/clawchat-mobile-commons";
import { motion, AnimatePresence } from "motion/react";

export const MomentsPage = () => {
  const [moments, setMoments] = useState<Moment[]>([]);
  const [activeCommentId, setActiveCommentId] = useState<string | null>(null);
  const [commentText, setCommentText] = useState("");
  const [isLoading, setIsLoading] = useState(true);

  const loadMoments = async () => {
    setIsLoading(true);
    const data = await MomentService.getMoments();
    setMoments(data);
    setIsLoading(false);
  };

  useEffect(() => {
    loadMoments();
  }, []);

  const handleLike = async (id: string) => {
    await MomentService.toggleLike(id, "me");
    loadMoments();
  };

  const submitComment = async (id: string) => {
    if (!commentText.trim()) return;
    await MomentService.addComment(id, "我", commentText);
    setCommentText("");
    setActiveCommentId(null);
    loadMoments();
  };

  return (
    <PageLayout title="朋友圈">
      <div className="flex flex-col flex-1 bg-bg-color min-h-full">
        {/* Cover Photo */}
        <div className="relative h-64 bg-gray-200">
          <img
            src="https://picsum.photos/seed/cover/800/400"
            alt="Cover"
            className="w-full h-full object-cover"
          />
          <div className="absolute -bottom-6 right-4 flex items-end gap-4">
            <span className="text-white font-bold text-lg mb-2 drop-shadow-md">
              Alex Chen
            </span>
            <Avatar
              src="https://picsum.photos/seed/alex/200/200"
              size="lg"
              className="w-16 h-16 rounded-xl border-2 border-bg-color"
            />
          </div>
        </div>

        {/* Moments List */}
        <div className="mt-12 pb-20">
          {isLoading ? (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
              <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
              <p className="text-[14px]">加载中...</p>
            </div>
          ) : moments.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
              <Camera className="w-12 h-12 mb-3 stroke-current opacity-40" />
              <p className="text-[14px]">暂无朋友圈动态</p>
            </div>
          ) : (
            moments.map((moment) => (
              <div
                key={moment.id}
                className="flex gap-3 p-4 border-b border-border-color"
              >
                <Avatar
                  src={moment.author.avatar}
                  size="md"
                  className="rounded-md shrink-0"
                />
                <div className="flex-1 w-0">
                  <h3 className="text-[#576B95] font-medium text-[15px] mb-1">
                    {moment.author.name}
                  </h3>
                  <p className="text-text-main text-[15px] leading-relaxed mb-2 break-words">
                    {moment.content}
                  </p>

                  {moment.images && moment.images.length > 0 && (
                    <div
                      className={cn(
                        "grid gap-1 mb-2",
                        moment.images.length === 1
                          ? "grid-cols-1 w-2/3"
                          : "grid-cols-3",
                      )}
                    >
                      {moment.images.map((img, i) => (
                        <img
                          key={i}
                          src={img}
                          alt="Moment"
                          className="w-full aspect-square object-cover bg-chat-other-bg"
                        />
                      ))}
                    </div>
                  )}

                  <div className="flex items-center justify-between mt-2 mb-2">
                    <span className="text-text-sub text-[12px]">
                      {new Date(moment.timestamp).toLocaleTimeString([], {
                        hour: "2-digit",
                        minute: "2-digit",
                      })}
                    </span>
                    <div className="flex gap-4 text-[#576B95]">
                      <button
                        onClick={() => handleLike(moment.id)}
                        className="flex items-center gap-1 active:opacity-50"
                      >
                        <Heart
                          className={cn(
                            "w-4 h-4",
                            moment.likes?.includes("me") &&
                              "fill-current text-rose-500",
                          )}
                        />
                      </button>
                      <button
                        onClick={() =>
                          setActiveCommentId(
                            activeCommentId === moment.id ? null : moment.id,
                          )
                        }
                        className="active:opacity-50"
                      >
                        <MessageCircle className="w-4 h-4" />
                      </button>
                    </div>
                  </div>

                  {/* Likes and Comments Area */}
                  {(moment.likes?.length > 0 ||
                    moment.comments?.length > 0) && (
                    <div className="bg-chat-other-bg rounded text-[13px] px-2 py-1.5 flex flex-col gap-1.5 mt-2">
                      {moment.likes?.length > 0 && (
                        <div className="flex items-start gap-1.5 text-[#576B95] font-medium">
                          <Heart className="w-3.5 h-3.5 mt-0.5 shrink-0" />
                          <span className="leading-relaxed">
                            {moment.likes.length} 人觉得很赞
                          </span>
                        </div>
                      )}

                      {moment.likes?.length > 0 &&
                        moment.comments?.length > 0 && (
                          <div className="h-[1px] bg-border-color/50 my-0.5" />
                        )}

                      {moment.comments?.length > 0 && (
                        <div className="flex flex-col gap-1 text-text-main">
                          {moment.comments.map((c) => (
                            <div key={c.id}>
                              <span className="text-[#576B95] font-medium">
                                {c.authorName}:{" "}
                              </span>
                              <span className="break-words">{c.content}</span>
                            </div>
                          ))}
                        </div>
                      )}
                    </div>
                  )}

                  {/* Inline Comment Input */}
                  <AnimatePresence>
                    {activeCommentId === moment.id && (
                      <motion.div
                        initial={{ opacity: 0, height: 0 }}
                        animate={{ opacity: 1, height: "auto" }}
                        exit={{ opacity: 0, height: 0 }}
                        className="overflow-hidden mt-3"
                      >
                        <div className="flex items-center gap-2 bg-chat-other-bg rounded-full px-3 py-1.5">
                          <input
                            type="text"
                            placeholder="评论..."
                            className="bg-transparent flex-1 outline-none text-[14px]"
                            value={commentText}
                            onChange={(e) => setCommentText(e.target.value)}
                            autoFocus
                          />
                          <button
                            className="text-primary-blue disabled:opacity-50 p-1"
                            disabled={!commentText.trim()}
                            onClick={() => submitComment(moment.id)}
                          >
                            <Send className="w-4 h-4" />
                          </button>
                        </div>
                      </motion.div>
                    )}
                  </AnimatePresence>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </PageLayout>
  );
};
